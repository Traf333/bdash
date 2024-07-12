import { Surreal, RecordId, Table } from "surrealdb.js";
export async function testConnection() {
  const db = new Surreal();

  // Connect to the database
  await db.connect("http://127.0.0.1:8000/rpc");

  // Select a specific namespace / database
  await db.use({
    namespace: "test",
    database: "test",
  });

  // Signin as a namespace, database, or root user
  await db.signin({
    username: "root",
    password: "root",
  });

  // Create a new person with a random id
  let created = await db.create("person", {
    title: "Founder & CEO",
    name: {
      first: "Tobie",
      last: "Morgan Hitchcock",
    },
    marketing: true,
  });
  console.log({ created });

  // Update a person record with a specific id
  let updated = await db.merge(new RecordId("person", "jaime"), {
    marketing: true,
  });
  console.log({ updated });

  // Select all people records
  let people = await db.select("person");
  console.log({ people });
  // Perform a custom advanced query
  let groups = await db.query(
    "SELECT marketing, count() FROM $tb GROUP BY marketing",
    {
      tb: new Table("person"),
    },
  );
  console.log({ groups });
}
