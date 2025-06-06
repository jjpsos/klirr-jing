#import "/crates/render/src/input.typ": make_invoice
#let invoice = make_invoice()
= #invoice.vendor.company_name
