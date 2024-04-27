import styles from "./MainPage.module.scss";
import { FC } from "react";

const MainPage: FC = () => {
  return (
    <section className={styles.main_page}>
      <div className={styles.content}>Main page</div>
    </section>
  );
};

export default MainPage;
