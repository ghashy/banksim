import "./App.scss";
import { Routes, BrowserRouter, Route } from "react-router-dom";
import MainLayout from "./Components/MainLayout";
import MainPage from "./Pages/MainPage";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route
          path="/"
          element={<MainLayout />}
        >
          <Route
            index
            element={<MainPage />}
          />
          <Route
            path="transactions"
            element={<div>transactions</div>}
          />
          <Route
            path="logs"
            element={<div>Logs</div>}
          />
          <Route
            path="about"
            element={<div>About</div>}
          />
          <Route
            path="*"
            element={<div>page not found</div>}
          />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
