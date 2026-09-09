export async function readUsers(request, query = "") {
  const parameters = new URLSearchParams(query);
  const sort = parameters.get("sort") || "id";
  return request.post("/users/read", {
    data: {
      search: parameters.get("search"),
      where_many: null,
      select: ["id", "login", "display_name", "is_banned"].map(field => ({ [field]: null })),
      order_by: {
        column: { [sort === "status" ? "is_banned" : sort]: null },
        order: parameters.get("direction") || "ascending"
      },
      pagination: {
        limit: Number(parameters.get("limit") ?? 20),
        offset: Number(parameters.get("offset") ?? 0)
      }
    }
  });
}

export async function usersReadPage(response) {
  const page = await response.json();
  return {
    ...page,
    items: page.items.map(row => Object.fromEntries(
      Object.entries(row).map(([field, value]) => [
        field,
        value && Object.hasOwn(value, "value") ? value.value : value
      ])
    ))
  };
}
