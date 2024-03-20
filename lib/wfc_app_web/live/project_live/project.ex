defmodule WfcAppWeb.ProjectLive.Project do
  use WfcAppWeb, :live_view
  require Logger

  alias WfcApp.Projects
  alias WfcApp.Projects.Project
  alias WfcApp.Rust

  @impl true
  def mount(params, _session, socket) do
    %{current_user: user} = socket.assigns
    Logger.debug "params info: #{inspect(params)}"

    project = Projects.get_project(params["project_id"])

    Logger.debug "project: #{inspect(project)}"

    image_list = project.wave
    |> String.split()
    |> Enum.map(fn x -> "#{project.images_path}#{x}" end)
    stream = Enum.zip(0..length(image_list),image_list)
    |> Enum.map(fn {k,v} -> %{:id => k, :tile => v} end)
    Logger.debug "STREAM: #{inspect(stream)}"
    {prob_map,prob_list} = create_prob_map("priv/static#{project.images_path}", project.probabilities)

    socket =
      socket
      |> push_event("grid-size", %{cols: project.x})
      |> assign(project: project)
      |> assign(form: to_form(%{}))
      |> assign(tt: "/images/final#{project.id}.png")
      |> assign(probs: prob_map)
      |> stream(:probs, prob_list)
      |> stream(:images, stream)
      |> assign(:s_tiles, project.starting_tiles)

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

  def create_prob_map(images_path, saved_probs) do
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
    Logger.debug "project-SOCKET: #{inspect(project)}"

    #Update probabilities
    nmap=socket.assigns.probs
    |> Enum.map(fn {k,v} -> {k,String.to_integer(params[k])} end)
    |> Enum.reduce(%{}, fn {k,v}, acc -> Map.put(acc,k,v) end)
    Logger.debug "NEW MAP: #{inspect(nmap)}"
    Projects.update_probabilities(project.id,nmap)
    s_tiles =
      socket.assigns.s_tiles
      |> Enum.map(fn {k,v} -> {k,Path.basename(v)} end)
      |> Enum.reduce(%{}, fn {k,v}, acc -> Map.put(acc,k,v) end)
    image_list = Rust.generate_image("priv/static#{project.jason_path}","priv/static#{project.images_path}","symmetry.json",{x,y},"priv/static/images/","final#{project.id}",nmap,s_tiles)
    Projects.update_wave(project.id,image_list,x,y)

    {:noreply, socket}
  end

  @impl true
  def handle_event("update_tile", params, socket) do
    %{project: project} = socket.assigns
    Projects.update_starting_tiles(project.id,Map.put(project.starting_tiles, socket.assigns.pos, "#{project.images_path}#{params["tile"]}"))
    socket = socket
      |> push_navigate(to: ~p"/project/#{socket.assigns.project.id}")
    Logger.debug "update_tile: #{inspect(socket)}"
    {:noreply, socket}
  end

  @impl true
  def handle_event("save_pos", params, socket) do
    Logger.debug "save_pos: #{inspect(params)}"
    {:noreply, socket |> assign(pos: params["pos"])}
  end

  @impl true
  def handle_event("debug", params, socket) do
    Logger.debug "DEBUG: #{inspect(params)}"
    {:noreply, socket}
  end

  @impl true
  def handle_event("clear_s_tiles", params, socket) do
    %{project: project} = socket.assigns
    Projects.update_starting_tiles(project.id,%{})
    {:noreply, socket |> push_navigate(to: ~p"/project/#{socket.assigns.project.id}")}
  end

  @impl true
  def handle_event("clear_s_tile", params, socket) do
    %{project: project} = socket.assigns
    Projects.update_starting_tiles(project.id,Map.drop(project.starting_tiles,[socket.assigns.pos]))
    {:noreply, socket |> push_navigate(to: ~p"/project/#{socket.assigns.project.id}")}
  end
end
