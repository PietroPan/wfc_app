defmodule WfcApp.Application do
  # See https://hexdocs.pm/elixir/Application.html
  # for more information on OTP Applications
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    children = [
      # Start the Telemetry supervisor
      WfcAppWeb.Telemetry,
      # Start the Ecto repository
      WfcApp.Repo,
      # Start the PubSub system
      {Phoenix.PubSub, name: WfcApp.PubSub},
      # Start Finch
      {Finch, name: WfcApp.Finch},
      # Start the Endpoint (http/https)
      WfcAppWeb.Endpoint
      # Start a worker by calling: WfcApp.Worker.start_link(arg)
      # {WfcApp.Worker, arg}
    ]

    # See https://hexdocs.pm/elixir/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :one_for_one, name: WfcApp.Supervisor]
    Supervisor.start_link(children, opts)
  end

  # Tell Phoenix to update the endpoint configuration
  # whenever the application is updated.
  @impl true
  def config_change(changed, _new, removed) do
    WfcAppWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end
