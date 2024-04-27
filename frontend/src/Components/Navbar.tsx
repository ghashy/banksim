import styles from "./Navbar.module.scss";
import { FC } from "react";

const Navbar: FC = () => {
  return (
    <nav className={styles.navbar}>
      <div>navbar</div>
    </nav>
  );
};

export default Navbar;
