import styles from "./Navbar.module.scss";
import { FC } from "react";

const NavInfo: FC = () => {
  return (
    <div className={styles.info_container}>
      <p className={styles.info_unit}>
        Store card: <span>8174505736614003</span>
      </p>
      <p className={styles.info_unit}>
        Store balance: <span>100_000</span>
      </p>
      <p className={styles.info_unit}>
        Bank emission: <span>432_129</span>
      </p>
    </div>
  );
};

export default NavInfo;
