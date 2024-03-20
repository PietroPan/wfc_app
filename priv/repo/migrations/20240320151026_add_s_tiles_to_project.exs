defmodule WfcApp.Repo.Migrations.AddSTilesToProject do
  use Ecto.Migration

  def change do
    alter table("projects") do
      add :starting_tiles, :map, default: %{}
    end
  end
end
