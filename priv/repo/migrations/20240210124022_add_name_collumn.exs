defmodule WfcApp.Repo.Migrations.AddNameCollumn do
  use Ecto.Migration

  def change do
    alter table("projects") do
      add :name, :string
    end
  end
end
