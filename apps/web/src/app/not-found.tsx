import Link from "next/link";

export default function NotFound() {
  return (
    <section className="route-state" aria-labelledby="not-found-heading">
      <p className="eyebrow">UrbanLens</p>
      <h1 id="not-found-heading">Workspace route not found</h1>
      <p>The current local foundation exposes the market map workspace only.</p>
      <Link className="button" href="/market-map">
        Open Market Map
      </Link>
    </section>
  );
}
