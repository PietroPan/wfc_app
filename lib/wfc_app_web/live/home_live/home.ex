defmodule WfcAppWeb.HomeLive.Home do
  use WfcAppWeb, :live_view
  require Logger

  alias WfcApp.Projects
  alias WfcApp.Projects.Project
  alias WfcApp.Rust

  @impl true
  def mount(_params, _session, socket) do
    %{current_user: user} = socket.assigns

    form =
      %Project{}
      |> Project.changeset(%{})
      |> to_form(as: "project")

    socket =
      socket
      |> assign(form: form)
      |> allow_upload(:tiles, accept: ~w(.zip), max_entries: 1)
      |> allow_upload(:images, accept: ~w(.zip), max_entries: 1)
      |> allow_upload(:rule_set, accept: ~w(.json), max_entries: 1)
      |> stream(:projects, Projects.list_projects(user.id))

    {:ok, socket}
  end

  @impl true
  def handle_event("validate", _params, socket) do
    {:noreply, socket}
  end

  @impl true
  def handle_event("save-post", %{"project" => project_params}, socket) do
    %{current_user: user} = socket.assigns

    jason_path = List.first(consume_files(socket,:rule_set))
    Logger.info "jason_path: #{inspect(jason_path)}"
    images_path = List.first(consume_files(socket,:tiles))
    Logger.info "images_path: #{inspect(images_path)}"
    unzip_path = String.to_charlist("priv/static#{images_path}ext")
    Logger.info "unzip_path: #{inspect(unzip_path)}"
    full_images_path = String.to_charlist("priv/static#{images_path}")
    Logger.info "full_images_path: #{inspect(full_images_path)}"

    case :zip.unzip(full_images_path, [{:cwd, unzip_path}]) do
      {:ok, _content} ->
        Logger.info "Unziped"
        full_images_path = "#{images_path}ext/images/"
        project_params
          |> Map.put("user_id", user.id)
          |> Map.put("jason_path", jason_path)
          |> Map.put("images_path", full_images_path)
          |> Projects.save()
          |> case do
            {:ok, project} ->
              image_list = Rust.generate_image("priv/static#{project.jason_path}","priv/static#{project.images_path}","symmetry.json",{10,10},"priv/static/images/","final#{project.id}",%{},%{},[])
              Logger.info "Image List: #{image_list}"
              Projects.update_wave(project.id,image_list,10,10)
              socket =
                socket
                |> put_flash(:info, "Project created successfully!")
                #|> redirect(to: ~p"/project/#{project.id}")

              {:noreply, socket}

            {:error, _changeset} ->
              {:noreply, socket |> put_flash(:error, "Something went wrong!")}
            end

      {:error, error} ->
        Logger.info "Failed to Unzip: #{inspect(error)}"
        {:noreply, socket}
    end
  end

  def consume_files(socket,content) do
    consume_uploaded_entries(socket, content, fn %{path: path}, _entry ->
      dest = Path.join([:code.priv_dir(:wfc_app), "static", "uploads", Path.basename(path)])
      Logger.info "Path: #{inspect(path)}"
      Logger.info "Dest: #{inspect(dest)}"
      File.cp!(path,dest)

      {:postpone, ~p"/uploads/#{Path.basename(dest)}"}
    end)
  end

  @impl true
  def handle_event("from-images-project", %{"project" => project_params}, socket) do
    %{current_user: user} = socket.assigns

    tile_x = String.to_integer(project_params["tile_x"])
    tile_y = String.to_integer(project_params["tile_y"])

    input_images_path = List.first(consume_files(socket, :images))
    Logger.info "input_images_path: #{inspect(input_images_path)}"
    unzip_path = String.to_charlist("priv/static#{input_images_path}ext")
    Logger.info "unzip_path: #{inspect(unzip_path)}"
    full_input_images_path = String.to_charlist("priv/static#{input_images_path}")
    Logger.info "full_images_path: #{inspect(full_input_images_path)}"

    case :zip.unzip(full_input_images_path, [{:cwd, unzip_path}]) do
      {:ok, _content} ->
        Logger.info "Unziped"
        full_input_images_path = "#{input_images_path}ext/in_images/"
        images_path = "#{input_images_path}tiles/"
        full_images_path = String.to_charlist("priv/static#{images_path}")
        Logger.info "Maked dir: #{:file.make_dir(full_images_path)}"
        new_p = project_params
            |> Map.put("user_id", user.id)
            |> Map.put("input_images_path", full_input_images_path)
            |> Map.put("images_path", images_path)
            |> Map.put("tile_x",tile_x)
            |> Map.put("tile_y",tile_y)
            |> Projects.save()
            |> case do
              {:ok, project} ->
                image_list = Rust.generate_image_i("priv/static#{project.input_images_path}",{tile_x,tile_y},{10,10},"priv/static#{project.images_path}","priv/static/images/","final#{project.id}",%{},%{},[],1)
                Logger.info "Image List: #{image_list}"
                Projects.update_wave(project.id,image_list,10,10)
                socket =
                  socket
                  |> put_flash(:info, "Project created successfully!")
                  #|> redirect(to: ~p"/project/#{project.id}")

                {:noreply, socket}
              {:error, _changeset} ->
                {:noreply, socket |> put_flash(:error, "Something went wrong!")}
              end
      {:error, error} ->
        Logger.info "Failed to Unzip: #{inspect(error)}"
        {:noreply, socket}
    end

    {:noreply, socket |> push_navigate(to: ~p"/home")}
  end

  @impl true
  def handle_event("debug", params, socket) do
    Logger.info "DEBUG: #{inspect(params)}"
    {:noreply, socket |> push_navigate(to: ~p"/home")}
  end

end
#force folder upload name?
#socket = socket |> push_event("debug", %{msg: "test0"})
