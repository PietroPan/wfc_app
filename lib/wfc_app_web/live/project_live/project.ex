defmodule WfcAppWeb.ProjectLive.Project do
  use WfcAppWeb, :live_view
  require Logger

  alias WfcApp.Projects
  alias WfcApp.Projects.Project
  alias WfcApp.Rust

  @impl true
  def mount(params, _session, socket) do
    Logger.debug "params info: #{inspect(params)}"

    project = Projects.get_project(params["project_id"])

    Logger.debug "project: #{inspect(project)}"

    socket =
      socket
      |> assign(project: project)
      |> assign(tt: "/images/final#{project.id}.png")

    {:ok, socket}
  end

  @impl true
  def handle_event("generate-image", params, socket) do
    %{project: project} = socket.assigns
    Rust.generate_image("priv/static#{project.jason_path}","priv/static#{project.images_path}","symmetry.json","priv/static/images/","final#{project.id}")
    {:noreply, socket}
  end
end
