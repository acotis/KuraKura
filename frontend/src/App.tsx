import { createHashRouter, RouterProvider } from "react-router-dom";
import Game from "./Game";
import Lab from "./Lab";
import { WebSocketProvider } from "./WebSocketContext";

const router = createHashRouter([
	{
		path: "/",
		element: <Lab />,
	},
	{
		path: "/game",
		element: <Game />,
	},
]);

function App() {
	return (
		<WebSocketProvider>
			<RouterProvider router={router} />
		</WebSocketProvider>
	);
}

export default App;
