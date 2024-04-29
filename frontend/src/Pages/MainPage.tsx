import AccountTable from "../Components/AccountTable";
import ActionButton from "../UI/ActionButton";
import styles from "./MainPage.module.scss";
import { FC } from "react";

const MainPage: FC = () => {
  return (
    <section className={styles.main_page}>
      <div className={styles.content}>
        <div className={styles.header_container}>
          <h2 className={styles.h2}>Accounts</h2>
          <div className={styles.action_buttons}>
            <ActionButton kind="new_transaction" />
            <ActionButton kind="new_account" />
          </div>
        </div>
        <AccountTable />
      </div>
    </section>
  );
};

export default MainPage;
