defmodule WfcApp.Repo.Migrations.ChangeProjectsTable do
  use Ecto.Migration

  def change do
    alter table("projects") do
      add :x, :integer
      add :y, :integer
      add :wave, :string
    end
  end
end
