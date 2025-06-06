#let make_invoice() = {
  (
  client: (
    company_name: "Holmes Ltd",
    contact_person: "Sherlock Holmes",
    organisation_number: "9876543-2101",
    postal_address: (
      city: "London",
      country: "England",
      street_address: (
        line_1: "221B Baker Street",
        line_2: "",
        line_3: "",
      ),
      zip: "NW1 6XE",
    ),
    vat_number: "GB987654321",
  ),
  information: (
    currency: "EUR",
    date: (
      day: 31,
      month: 5,
      year: 2025,
    ),
    identifier: "INV-2025-001",
    purchase_order: "PO-12345",
    terms: (
      net: 30,
    ),
  ),
  line_items: (
    Expenses: [
      (
        currency: "EUR",
        name: "Consulting services",
        quantity: 10.0,
        transaction_date: (
          day: 31,
          month: 5,
          year: 2025,
        ),
        unit_price: 50.0,
      ),
    ],
  ),
  vendor: (
    company_name: "Bra Detektiv AB",
    contact_person: "Ture Sventon",
    organisation_number: "556123-4567",
    postal_address: (
      city: "Stockholm",
      country: "Sweden",
      street_address: (
        line_1: "Storgatan 45",
        line_2: "4 tr",
        line_3: "Apt 12",
      ),
      zip: "114 32",
    ),
    vat_number: "SE556123456701",
  ),
)
}
