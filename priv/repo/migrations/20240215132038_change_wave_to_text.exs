defmodule WfcApp.Repo.Migrations.ChangeWaveToText do
  use Ecto.Migration

  def change do
    alter table("projects") do
      modify :wave, :text
    end
  end
end
