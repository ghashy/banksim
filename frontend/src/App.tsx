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
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
