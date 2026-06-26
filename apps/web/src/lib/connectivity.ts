import { GraphQLClient, gql } from "graphql-request";

export type Connectivity = {
  service: string;
  status: string;
  databaseReachable: boolean;
  migrationsApplied: boolean;
};

type ConnectivityResponse = {
  connectivity: Connectivity;
};

export const CONNECTIVITY_QUERY = gql`
  query ConnectivityProof {
    connectivity {
      service
      status
      databaseReachable
      migrationsApplied
    }
  }
`;

export function getGraphqlUrl() {
  return process.env.NEXT_PUBLIC_GRAPHQL_URL ?? "http://localhost:8080/graphql";
}

export async function fetchConnectivity(): Promise<Connectivity> {
  const requestId =
    typeof globalThis.crypto?.randomUUID === "function"
      ? globalThis.crypto.randomUUID()
      : `${Date.now()}`;
  const client = new GraphQLClient(getGraphqlUrl(), {
    headers: {
      "x-request-id": `web-connectivity-${requestId}`,
    },
  });
  const response = await client.request<ConnectivityResponse>(CONNECTIVITY_QUERY);

  return response.connectivity;
}
