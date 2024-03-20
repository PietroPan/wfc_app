defmodule WfcApp do
  @moduledoc """
  WfcApp keeps the contexts that define your domain
  and business logic.

  Contexts are also responsible for managing your data, regardless
  if it comes from the database, an external API or others.
  """

  def testR do
    WfcApp.Rust.add(1,4)
  end

  def testFiles(dir) do
    File.dir?(dir)
  end
end

# "priv/static/uploads/live_view_upload-1707672399-364678243519-2ext"
