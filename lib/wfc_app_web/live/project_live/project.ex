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

    socket =
      socket
      |> assign(project: project)
      |> assign(tt: "/images/final#{project.id}.png")

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
    Rust.generate_image("priv/static#{project.jason_path}","priv/static#{project.images_path}","symmetry.json","priv/static/images/","final#{project.id}")
    {:noreply, socket}
  end
end
