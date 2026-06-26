import { describe, expect, it, vi } from "vitest";

import HomePage from "./page";

const { redirect } = vi.hoisted(() => ({
  redirect: vi.fn(),
}));

vi.mock("next/navigation", () => ({
  redirect,
}));

describe("HomePage", () => {
  it("redirects the root route to the market map workspace", () => {
    HomePage();

    expect(redirect).toHaveBeenCalledWith("/market-map");
  });
});
