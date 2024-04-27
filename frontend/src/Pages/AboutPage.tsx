import styles from "./AboutPage.module.scss";
import { FC } from "react";

const AboutPage: FC = () => {
  return (
    <section className={styles.about_page}>
      <div className={styles.content}>About</div>
    </section>
  );
};

export default AboutPage;
