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
      ),
      zip: "NW1 6XE",
    ),
    vat_number: "GB987654321",
  ),
  information: (
    date: "2025-05-31",
    due_date: "2025-06-30",
    footer_text: "Reverse VAT according to chapter 1 2§ first section 4b in the VAT regulation.",
    identifier: "INV-9876",
    purchase_order: "PO-12345",
    terms: (
      net: 30,
    ),
  ),
  line_items: (
    expenses: (
      (
        currency: "EUR",
        name: "Breakfast",
        quantity: 1.0,
        total_cost: 1602.25,
        transaction_date: "2025-05-20",
        unit_price: 1602.25,
      ),
      (
        currency: "EUR",
        name: "Coffee",
        quantity: 2.0,
        total_cost: 9.392,
        transaction_date: "2025-05-31",
        unit_price: 4.696,
      ),
      (
        currency: "EUR",
        name: "Sandwich",
        quantity: 1.0,
        total_cost: 8.218,
        transaction_date: "2025-05-31",
        unit_price: 8.218,
      ),
    ),
    service: none,
  ),
  payment_info: (
    bank_name: "SEB",
    bic: "ESSESESS",
    currency: "EUR",
    iban: "SE21 9000 0123 9876 5432 1009",
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
      ),
      zip: "114 32",
    ),
    vat_number: "SE556123456701",
  ),
)
}
