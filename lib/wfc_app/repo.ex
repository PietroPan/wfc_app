defmodule WfcApp.Repo do
  use Ecto.Repo,
    otp_app: :wfc_app,
    adapter: Ecto.Adapters.Postgres
end
