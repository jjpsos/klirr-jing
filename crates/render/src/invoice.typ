#import "/crates/render/src/input.typ": make_invoice
#let invoice = make_invoice()

// ** Vendor Data Variables **
#let vendor_company_name = invoice.vendor.company_name
#let vendor_addr_line1 = invoice.vendor.postal_address.street_address.line_1
#let vendor_addr_line2 = invoice.vendor.postal_address.street_address.line_2
#let vendor_zip = invoice.vendor.postal_address.zip
#let vendor_city = invoice.vendor.postal_address.city
#let vendor_country = invoice.vendor.postal_address.country
#let bic = invoice.payment_info.bic
#let iban = invoice.payment_info.iban
#let org_no = invoice.vendor.organisation_number
#let vat_no = invoice.vendor.vat_number
#let bank_name = invoice.payment_info.bank_name
#let our_reference = invoice.vendor.contact_person

// ** Invoice Data Variables **
#let invoice_no = invoice.information.identifier
#let purchase_order = invoice.information.purchase_order
#let currency = invoice.payment_info.currency
#let client_attention = invoice.client.contact_person
#let payment_terms = invoice.information.terms.net
#let invoice_date = invoice.information.date
#let due_date = invoice.information.due_date
#let footer_text = invoice.information.footer_text
#let line_items = invoice.line_items.expenses


// ** Client Data Variables **
#let client_name = invoice.client.company_name
#let client_addr_line1 = invoice.client.postal_address.street_address.line_1
#let client_addr_line2 = invoice.client.postal_address.street_address.line_2
#let client_zip = invoice.client.postal_address.zip
#let client_city = invoice.client.postal_address.city
#let client_country = invoice.client.postal_address.country
#let client_vat = invoice.client.vat_number


#let hline(
  length: 100%,
  thickness: 0.1pt,
  color: rgb(10, 10, 10),
) = {
  block[
    #line(
      length: length,
      stroke: (thickness: thickness, paint: color),
    )
  ]
}

#let double-line(
  length: 100%,
  thickness: 0.1pt,
  color: rgb(10, 10, 10),
) = {
  block[
    #hline(length: length, thickness: thickness, color: color)
    #v(-10pt)
    #hline(length: length, thickness: thickness, color: color)
  ]
}

// Function to format numbers to two decimals
#let format_amount(amount, currency) = {
  let amt = amount * 1.0
  let integer = calc.floor(amt)
  let frac = int(calc.round((amt - integer) * 100, digits: 0))
  let frac_str = str(frac)
  if frac < 10 { frac_str = "0" + frac_str }
  let without_currency = str(integer) + "." + frac_str
  without_currency + currency
}


#set text(size: 14pt, weight: "bold")

// Page setup: A4 paper, custom margins, and footer for contact details
#set page(
  margin: (top: 2cm, bottom: 2cm, left: 2.5cm, right: 2.5cm),
  footer: [
    #set text(size: 12pt)
    #footer_text
  ],
)

// ** Invoice Header Section **
= #vendor_company_name

#set text(size: 11pt, weight: "regular")

// Recipient address block
#align(left)[
  *To:*\
  #client_name\
  #client_addr_line1\
  #client_addr_line2
  *VAT*:\
  #client_vat
]

#align(right)[
  // align the following block to the right margin
  Invoice no: *#invoice_no* \
  Invoice date: #invoice_date \
  Due date: #due_date \
  For the attention of *#client_attention* \
  Our reference: #our_reference \
  Terms: Net #payment_terms
]

// ** Invoice Items Table **
#double-line()
// Calculate total in a scripting block
#let grand_total
#{
  grand_total = 0.0
  for it in line_items { grand_total = grand_total + it.total_cost }
}
#v(-5pt)
#table(
  columns: (1fr, auto, auto, auto, auto),
  align: (left, right, center, center, right),
  stroke: none,
  table.header(
    [*Description*],
    [*Quantity*],
    [*Unit Price*],
    [*Total Cost*],
    [*Transaction Date*],
  ),
  table.hline(stroke: 0.1pt),
  ..for row in line_items {
    (
      row.name,
      str(row.quantity),
      format_amount(row.unit_price, row.currency),
      format_amount(row.total_cost, row.currency),
      row.transaction_date,
      table.hline(stroke: (thickness: 0.1pt, dash: "dashed")),
    )
  }
)
// Grand Total Row
#align(right)[
  #set text(weight: "bold")
  Grand Total: #format_amount(grand_total, currency)
]
#v(-5pt)
#double-line()

#table(
  columns: (auto, auto, auto),
  align: (left, left, left),
  stroke: none,
  // Footer: Address | Bank Info | Company numbers
  [*Address:*], [*IBAN*], [*Org. No.*],
  [#vendor_company_name], [#iban], [#org_no],
  [#vendor_addr_line1], [*Bank:*], [*VAT. No.*],
  [#vendor_addr_line2], [#bank_name], [#vat_no],
  [#vendor_zip], [#vendor_city], [*BIC:*], [],
  [#vendor_country], [#bic], []
)
