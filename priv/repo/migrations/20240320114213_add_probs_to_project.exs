defmodule WfcApp.Repo.Migrations.AddProbsToProject do
  use Ecto.Migration

  def change do
    alter table("projects") do
      add :probabilities, :map, default: %{}
    end
  end
end
