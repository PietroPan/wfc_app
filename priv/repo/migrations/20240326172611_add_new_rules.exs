defmodule WfcApp.Repo.Migrations.AddNewRules do
  use Ecto.Migration

  def change do
    alter table("projects") do
      add :new_rules, {:array, :string}, default: []
    end
  end
end
