defmodule WfcAppWeb.ProjectLive.Project do
  use WfcAppWeb, :live_view
  require Logger

  alias WfcApp.Projects
  alias WfcApp.Rust

  # path = /app/lib/wfc_app-0.1.0/priv/static/...

  @impl true
  def mount(params, _session, socket) do
    %{current_user: user} = socket.assigns
    Logger.info "params info: #{inspect(params)}"

    project = Projects.get_project(params["project_id"])

    Logger.info "project: #{inspect(project)}"

    image_list = project.wave
    |> String.split()
    |> Enum.map(fn x -> "#{project.images_path}#{x}" end)
    stream = Enum.zip(0..length(image_list),image_list)
    |> Enum.map(fn {k,v} -> %{:id => k, :tile => v} end)
    Logger.info "STREAM: #{inspect(stream)}"
    {prob_map,prob_list} = create_prob_map("priv/static#{project.images_path}", project.probabilities)

    Logger.info "NEW RULES: #{inspect(project.new_rules)}"
    n_rules = project.new_rules
    |> Enum.reduce([], fn rule, acc ->
      [tA,dir,tB] = rule |> String.split
      [%{id: length(acc),rule: %{tileA: tA, dir: dir_to_string(dir),tileB: tB}}|acc]
    end)
    |> Enum.sort()

    socket =
      socket
      |> push_event("grid-size", %{cols: project.x})
      |> assign(project: project)
      |> assign(form: to_form(%{}))
      |> assign(tt: "/images/final#{project.id}.png")
      |> assign(probs: prob_map)
      |> stream(:probs, prob_list)
      |> stream(:images, stream)
      |> stream(:n_rules, n_rules)
      |> assign(:s_tiles, project.starting_tiles)
      |> assign(:n_tries, project.n_tries)
      |> assign(tileA: "/images/grid.png" )
      |> assign(tileB: "/images/grid.png" )
      |> allow_upload(:n_rule_set, accept: ~w(.json), max_entries: 1)

    project_user = Projects.get_correspondent_user(project.id)
    cond do
      project_user == user.id ->
        {:ok, socket}
      true ->
        socket =
          socket
          |> put_flash(:error, "Can't access this resource!")
          |> push_navigate(to: ~p"/home")

        {:ok, socket}
    end

  end

  def default_image({x,y}) do
    List.duplicate("/images/grid.png", x*y)
    |> Enum.reduce([], fn tile, acc ->
      [%{id: length(acc),tile: tile}|acc]
    end)
    |> Enum.sort()
  end

  @impl true
  def handle_params(_params, _uri, socket) do
    %{project: project} = socket.assigns
    {:noreply, socket |> push_event("grid-size", %{cols: project.x})}
  end

  def dir_to_string(dir) do
    case dir do
      "0" -> "Up"
      "1" -> "Right"
      "2" -> "Down"
      "3" -> "Left"
      _ -> "ERROR"
    end
  end

  def create_prob_map(images_path, saved_probs) do
    Logger.debug("TT: #{inspect(images_path)}")
    {:ok, images} = File.ls(images_path)
    prob_list = images
    |> Enum.sort()
    |> Enum.map(fn x ->
       if Map.has_key?(saved_probs,x) do
        %{id: x, prob: saved_probs[x]}
       else
        %{id: x, prob: 100}
       end
    end)
    prob_map = prob_list
    |> Enum.reduce(%{}, fn x, acc -> Map.put(acc,x.id,x.prob) end)
    {prob_map,prob_list}
  end

  @impl true
  def handle_event("generate_image", params, socket) do
    %{project: project} = socket.assigns

    x = String.to_integer(params["x"])
    y = String.to_integer(params["y"])
    n_tries = String.to_integer(params["n_tries"])
    Logger.info "project-SOCKET: #{inspect(project)}"

    #Update probabilities
    nmap=socket.assigns.probs
    |> Enum.map(fn {k,_v} -> {k,String.to_integer(params[k])} end)
    |> Enum.reduce(%{}, fn {k,v}, acc -> Map.put(acc,k,v) end)
    Logger.info "NEW MAP: #{inspect(nmap)}"
    Projects.update_probabilities(project.id,nmap)
    s_tiles =
      socket.assigns.s_tiles
      |> Enum.map(fn {k,v} -> {k,Path.basename(v)} end)
      |> Enum.reduce(%{}, fn {k,v}, acc -> Map.put(acc,k,v) end)
    image_list = cond do
      project.jason_path != nil ->
        Rust.generate_image("priv/static#{project.jason_path}","priv/static#{project.images_path}","symmetry.json",{x,y},"priv/static/images/","final#{project.id}",nmap,s_tiles,project.new_rules)
      true ->
        Rust.generate_image_i("priv/static#{project.input_images_path}",{project.tile_x,project.tile_y},{x,y},"priv/static#{project.images_path}","priv/static/images/","final#{project.id}",nmap,s_tiles,project.new_rules,project.n_tries)
      end
    socket = case image_list do
      "" -> socket |> put_flash(:error, "Couldn't find an answer!")
      _ ->
        Projects.update_wave(project.id,image_list,x,y,n_tries)
        socket
      end
    {:noreply, socket |> redirect(to: ~p"/project/#{socket.assigns.project.id}")}
  end

  @impl true
  def handle_event("update_tile", params, socket) do
    %{project: project} = socket.assigns
    Projects.update_starting_tiles(project.id,Map.put(project.starting_tiles, socket.assigns.pos, "#{project.images_path}#{params["tile"]}"))
    socket = socket
      |> push_navigate(to: ~p"/project/#{socket.assigns.project.id}")
    Logger.info "update_tile: #{inspect(socket)}"
    {:noreply, socket}
  end

  @impl true
  def handle_event("save_pos", params, socket) do
    Logger.info "save_pos: #{inspect(params)}"
    {:noreply, socket |> assign(pos: params["pos"])}
  end

  @impl true
  def handle_event("debug", params, socket) do
    Logger.info "DEBUG: #{inspect(params)}"
    {:noreply, socket}
  end

  @impl true
  def handle_event("remove_rule", params, socket) do
    Projects.remove_rule(socket.assigns.project.id,String.to_integer(params["id"]))
    {:noreply, socket |> push_navigate(to: ~p"/project/#{socket.assigns.project.id}")}
  end

  @impl true
  def handle_event("clear_s_tiles", _params, socket) do
    %{project: project} = socket.assigns
    Projects.update_starting_tiles(project.id,%{})
    {:noreply, socket |> push_navigate(to: ~p"/project/#{socket.assigns.project.id}")}
  end

  @impl true
  def handle_event("clear_s_tile", _params, socket) do
    %{project: project} = socket.assigns
    Projects.update_starting_tiles(project.id,Map.drop(project.starting_tiles,[socket.assigns.pos]))
    {:noreply, socket |> push_navigate(to: ~p"/project/#{socket.assigns.project.id}")}
  end

  @impl true
  def handle_event("delete_project", _params, socket) do
    %{project: project} = socket.assigns
    upload_path = Path.split(project.images_path)|> Enum.reverse() |> tl() |> Enum.reverse() |> Path.join()
    Projects.delete_project(project.id)
    File.rm!("priv/static#{project.jason_path}")
    File.rm_rf!("priv/static#{project.images_path}")
    File.rmdir!("priv/static#{upload_path}")
    File.rm!("priv/static#{upload_path |> String.slice(0..String.length(upload_path)-4)}")
    {:noreply, socket |> put_flash(:info, "Project deleted successfully!") |> redirect(to: ~p"/home")}
  end

  @impl true
  def handle_event("save_tile_a", params, socket) do
    %{project: project} = socket.assigns
    {:noreply, socket |> assign(tileA: "#{project.images_path}#{params["tile"]}") |> push_patch(to: ~p"/project/#{project.id}")}
  end

  @impl true
  def handle_event("save_tile_b", params, socket) do
    %{project: project} = socket.assigns
    {:noreply, socket |> assign(tileB: "#{project.images_path}#{params["tile"]}") |> push_patch(to: ~p"/project/#{project.id}") }
  end

  @impl true
  def handle_event("add_rule", params, socket) do
    %{project: project} = socket.assigns
    tileA = Path.basename(socket.assigns.tileA)
    tileB = Path.basename(socket.assigns.tileB)
    socket = cond do
      tileA != "default.png" and tileB != "default.png" ->
        Projects.add_rule(project.id,tileA,params["dir"],tileB)
        socket |> push_navigate(to: ~p"/project/#{socket.assigns.project.id}")
      true ->
        socket |> put_flash(:error, "Please select both tiles!")
        |> push_patch(to: ~p"/project/#{project.id}")
    end
      {:noreply, socket}
  end

  def dir_to_phrase(dir) do
    case dir do
      "Up" -> "can be above"
      "Right" -> "can be to the right of"
      "Down" -> "can be below"
      "Left" -> "can be to the left of"
      _ -> :invalid_direction
    end
  end

  @impl true
  def handle_event("clear_grid", _params, socket) do
    %{project: project} = socket.assigns
    socket =
      socket |> stream(:images, default_image({project.x,project.y}), reset: true)
             |> push_patch(to: ~p"/project/#{project.id}")

    {:noreply, socket}
  end

  @impl true
  def handle_event("clear_probs", _params, socket) do
    %{project: project} = socket.assigns
    {n_prob_map,n_prob_list}=create_prob_map("priv/static#{project.images_path}", %{})
    Projects.update_probabilities(project.id,n_prob_map)
    socket =
      socket |> stream(:probs, n_prob_list, reset: true)
             |> assign(probs: n_prob_map)
             |> push_patch(to: ~p"/project/#{project.id}")
    {:noreply, socket}
  end

  @impl true
  def handle_event("validate", _params, socket) do
    {:noreply, socket}
  end

  @impl true
  def handle_event("update_rule_set",_params,socket) do
    %{project: project} = socket.assigns

    n_jason_path =List.first(consume_files(socket,:n_rule_set))
    File.rm!("priv/static#{project.jason_path}")
    Projects.update_rule_set(project.id, n_jason_path)
    {:noreply, socket |> put_flash(:info, "Rule Set updated successfully!") |> push_navigate(to: ~p"/project/#{project.id}")}
  end

  def consume_files(socket,content) do
    consume_uploaded_entries(socket, content, fn %{path: path}, _entry ->
      dest = Path.join([:code.priv_dir(:wfc_app), "static", "uploads", Path.basename(path)])
      File.cp!(path,dest)

      {:postpone, ~p"/uploads/#{Path.basename(dest)}"}
    end)
  end
end
