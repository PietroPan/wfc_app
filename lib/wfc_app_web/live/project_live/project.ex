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

    socket =
      socket
      |> push_event("grid-size", %{cols: project.x})
      |> assign(project: project)
      |> assign(form: to_form(%{}))
      |> assign(tt: "/images/final#{project.id}.png")
      |> stream_configure(:images, dom_id: &("image-#{elem(&1,0)}"))
      |> stream(:images, stream)

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

  @impl true
  def handle_event("generate-image", params, socket) do
    x = String.to_integer(params["x"])
    y = String.to_integer(params["y"])
    %{project: project} = socket.assigns
    Logger.debug "project-SOCKET: #{inspect(project)}"
    image_list = Rust.generate_image("priv/static#{project.jason_path}","priv/static#{project.images_path}","symmetry.json",{x,y},"priv/static/images/","final#{project.id}")
    Projects.update_wave(project.id,image_list,x,y)
    #JS.dispatch("test", to: "#tile_grid", detail: %{cols: project.x})
    {:noreply, socket}
  end

end
