// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use dbus_tree::{Factory, MTSync, Method};

use crate::{
    dbus_api::{api::report_3_0::methods::get_report, types::TData},
    engine::Engine,
};

pub fn get_report_method<E>(
    f: &Factory<MTSync<TData<E>>, TData<E>>,
) -> Method<MTSync<TData<E>>, TData<E>>
where
    E: 'static + Engine,
{
    f.method("GetReport", (), get_report)
        .in_arg(("name", "s"))
        // The report is a JSON object.
        //
        // s: string representation of a JSON object containing the report
        //
        // Rust representation: serde_json::Value
        .out_arg(("result", "s"))
        .out_arg(("return_code", "q"))
        .out_arg(("return_string", "s"))
}
