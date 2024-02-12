defmodule WfcApp.Projects do
  import Ecto.Query, warn: false

  alias WfcApp.Repo
  alias WfcApp.Projects.Project

  def save(project_params) do
    %Project{}
    |> Project.changeset(project_params)
    |> Repo.insert()
  end

  def list_projects(user_id) do
    query =
      from p in Project,
      where: p.user_id == ^user_id,
      select: p,
      order_by: [desc: :inserted_at]

    Repo.all(query)
  end

  def get_project(project_id) do
    p = Repo.get(Project, project_id)
    %{id: p.id, name: p.name, images_path: p.images_path, jason_path: p.jason_path}
  end

  def get_correspondent_user(project_id) do
    p = Repo.get(Project, project_id)
    p.user_id
  end
end
