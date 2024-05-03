import { useSelector } from "react-redux";
import styles from "./Navbar.module.scss";
import { FC } from "react";
import { RootState } from "../state/store";
import { IStoreInfo } from "../types";

const NavInfo: FC = () => {
  const store_info = useSelector<RootState, IStoreInfo>(
    (state) => state.store_info
  );

  return (
    <div className={styles.info_container}>
      <p className={styles.info_unit}>
        Store card: <span>{store_info.card}</span>
      </p>
      <p className={styles.info_unit}>
        Store balance: <span>{store_info.balance}</span>
      </p>
      <p className={styles.info_unit}>
        Bank emission: <span>{store_info.emission}</span>
      </p>
    </div>
  );
};

export default NavInfo;
