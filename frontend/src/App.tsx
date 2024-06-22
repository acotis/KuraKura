import { createHashRouter, RouterProvider } from "react-router-dom";
import Game from "./Game";
import Home from "./Home";
import { WebSocketProvider } from "./WebSocketContext";

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
  return (
    <WebSocketProvider>
      <RouterProvider router={router}></RouterProvider>
    </WebSocketProvider>
  );
}

export default App;
