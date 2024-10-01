import { createResource, Match, Switch, type Component } from "solid-js";

const fetchData = async () => {
  const response = await fetch(`http://127.0.0.1:4000/welcome`);
  return response.json();
};

const App: Component = () => {
  const [data] = createResource<{ message: string }>(fetchData);

  return (
    <>
      <h1>Welcome to Gym Ops</h1>
      <Switch>
        <Match when={data.loading}>
          <p>Loading...</p>
        </Match>
        <Match when={data.error}>
          <p>Error : {data.error}</p>
        </Match>
        <Match when={data()}>
          <p>{data().message}</p>
        </Match>
      </Switch>
    </>
  );
};

export default App;
