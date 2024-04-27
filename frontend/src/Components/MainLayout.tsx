import { FC } from "react";
import Navbar from "./Navbar";
import { Outlet } from "react-router-dom";

const MainLayout: FC = () => {
  return (
    <>
      <Navbar />
      <Outlet />
    </>
  );
};

export default MainLayout;
