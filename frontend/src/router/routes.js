const routes = [
  {
    path: "/",
    component: () => import("layouts/MainLayout.vue"),
    children: [
      { path: "", component: () => import("pages/Index.vue") },
      { path: "shop", component: () => import("pages/Shop.vue") },
      { path: "market", component: () => import("pages/Market.vue") },
      { path: "community", component: () => import("pages/Community.vue") },
      { path: "downloads", component: () => import("pages/Downloads.vue") },
      {
        path: "newsview",
        name: "newsview",
        component: () => import("pages/NewsView.vue"),
      },
      { path: "admin", component: () => import("src/pages/Admin.vue") },
    ],
  },

  // Always leave this as last one,
  // but you can also remove it
  {
    path: "/:catchAll(.*)*",
    component: () => import("pages/Error404.vue"),
  },
];

export default routes;
