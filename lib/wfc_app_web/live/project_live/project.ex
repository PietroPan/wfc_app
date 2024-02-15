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
      |> assign(project: project)
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
    %{project: project} = socket.assigns
    image_list = Rust.generate_image("priv/static#{project.jason_path}","priv/static#{project.images_path}","symmetry.json",{project.x,project.y},"priv/static/images/","final#{project.id}")
    Projects.update_wave(project.id,image_list)
    {:noreply, socket}
  end
end
