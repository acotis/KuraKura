import { createHashRouter, RouterProvider } from "react-router-dom";
import Lab from "./Lab";
import { WebSocketProvider } from "./WebSocketContext";

const router = createHashRouter([
	{
		path: "/",
		element: <Lab />,
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
