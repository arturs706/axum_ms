import styles from "../css/nav.module.css";

export default function Nav() {
  return (
    <div class={styles.nav}>
        <h3>Dashboard</h3>
        <a href="/">Home</a>
        <a href="/lettings">Lettings</a>
        <a href="/sales">Sales</a>
        <a href="/admin">Admin</a>
    </div>
  );
}
