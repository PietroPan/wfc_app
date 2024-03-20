defmodule WfcApp.Projects.Project do
  use Ecto.Schema
  import Ecto.Changeset

  alias WfcApp.Accounts.User

  schema "projects" do
    field :images_path, :string
    field :jason_path, :string
    field :name, :string
    field :x, :integer, default: 10
    field :y, :integer, default: 10
    field :wave, :string, default: ""
    field :probabilities, :map, default: %{}
    field :starting_tiles, :map, default: %{}
    belongs_to :user, User

    timestamps()
  end

  @doc false
  def changeset(project, attrs) do
    project
    |> cast(attrs, [:images_path, :jason_path, :name, :x, :y, :wave, :user_id, :probabilities, :starting_tiles])
    |> validate_required([:images_path, :jason_path, :name, :x, :y, :wave, :user_id])
  end
end
