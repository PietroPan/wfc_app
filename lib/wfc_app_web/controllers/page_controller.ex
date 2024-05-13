defmodule WfcAppWeb.PageController do
  use WfcAppWeb, :controller

  def home(conn, _params) do
    # The home page is often custom made,
    # so skip the default app layout.
    redirect(conn, to: ~p"/users/log_in")
  end
end
