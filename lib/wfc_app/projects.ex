defmodule WfcApp.Projects do
  import Ecto.Query, warn: false
  require Logger

  alias WfcApp.Repo
  alias WfcApp.Projects.Project

  def save(project_params) do
    Logger.debug "IM INNNNNNNNNNNNNNNNNN: #{inspect(project_params)}"

    %Project{}
    |> Project.changeset(project_params)
    |> Repo.insert()
  end

  def update_wave(id,wave,x,y) do
  Repo.get!(Project, id)
  |> Ecto.Changeset.change(wave: wave)
  |> Ecto.Changeset.change(x: x)
  |> Ecto.Changeset.change(y: y)
  |> Repo.update!()
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
    Repo.get(Project, project_id)
  end


  def get_correspondent_user(project_id) do
    p = Repo.get(Project, project_id)
    p.user_id
  end
end
