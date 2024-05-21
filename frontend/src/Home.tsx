export default function Home() {
  return (
    <main className="fcc gap-4 p-8">
      <h1 className="font-bold text-3xl">kurakura!</h1>
      <div className="card bg-base-100 fc gap-4 shadow-xl p-8">
        <h2 className="font-bold text-xl">Start a game</h2>
        <label className="input input-bordered flex items-center gap-2">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            strokeWidth={1.5}
            stroke="currentColor"
            className="w-6 h-6"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              d="M17.982 18.725A7.488 7.488 0 0 0 12 15.75a7.488 7.488 0 0 0-5.982 2.975m11.963 0a9 9 0 1 0-11.963 0m11.963 0A8.966 8.966 0 0 1 12 21a8.966 8.966 0 0 1-5.982-2.275M15 9.75a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z"
            />
          </svg>
          <input type="text" className="grow" placeholder="Username" />
        </label>
        <button className="btn btn-primary">Start</button>
      </div>
    </main>
  );
}
