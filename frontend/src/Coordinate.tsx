export interface CoordinateProps {
	tileSize: number;
	row?: number;
	column?: number;
}

export default function Coordinate({ tileSize, row, column }: CoordinateProps) {
	return (
		<div
			className="absolute z-9 flex items-center justify-center text-black"
			style={{
				left: column !== undefined ? column * tileSize : -32,
				top: row !== undefined ? row * tileSize : -32,
				width: column !== undefined ? tileSize : 32,
				height: row !== undefined ? tileSize : 32,
			}}
		>
			{row !== undefined
				? row + 1
				: column !== undefined
					? String.fromCharCode(97 + column)
					: undefined}
		</div>
	);
}
