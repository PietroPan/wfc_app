defmodule WfcApp.Projects.Project do
  use Ecto.Schema
  import Ecto.Changeset

  alias WfcApp.Accounts.User

  schema "projects" do
    field :images_path, :string
    field :jason_path, :string
    belongs_to :user, User

    timestamps()
  end

  @doc false
  def changeset(project, attrs) do
    project
    |> cast(attrs, [:images_path, :jason_path, :user_id])
    |> validate_required([:images_path, :jason_path, :user_id])
  end
end
