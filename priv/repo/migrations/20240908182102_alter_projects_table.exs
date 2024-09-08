defmodule WfcApp.Repo.Migrations.AlterProjectsTable do
  use Ecto.Migration

  def change do
    alter table("projects") do
      add :n_tries, :integer, default: 1
      add :input_images_path, :string
      add :tile_x, :integer
      add :tile_y, :integer
    end
  end
end
