@spec fetch_in(
          list({String.t(), list({String.t()}), list({String.t(), list({String.t(), list()})})}),
          list(String.t())
        ) ::
          {:ok, list({String.t(), list(), list()})} | :error
  def(fetch_in(tree, keypath) when is_list(keypath)) do
    case do_select(tree, keypath) do
      [] -> :error
      found -> {:ok, found}
    end
  end

is that too crazy
I mean
