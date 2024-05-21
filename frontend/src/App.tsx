import { createHashRouter, RouterProvider } from "react-router-dom";
import Game from "./Game";
import Home from "./Home";

const router = createHashRouter([
  {
    path: "/",
    element: <Home />,
  },
  {
    path: "/game",
    element: <Game />,
  },
]);

function App() {
  return <RouterProvider router={router}></RouterProvider>;
}

export default App;
