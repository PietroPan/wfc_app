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
    field :new_rules, {:array, :string}, default: []
    field :n_tries, :integer, default: 1
    field :input_images_path, :string
    field :tile_x, :integer, default: 16
    field :tile_y, :integer, default: 16
    belongs_to :user, User

    timestamps()
  end

  @doc false
  def changeset(project, attrs) do
    project
    |> cast(attrs, [:images_path, :jason_path, :name, :x, :y, :wave, :user_id, :probabilities, :starting_tiles, :new_rules, :n_tries, :input_images_path, :tile_x, :tile_y])
    |> validate_required([:images_path, :name, :user_id])
  end
end
