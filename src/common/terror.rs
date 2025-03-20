// Executor error codes.

use lazy_static::lazy_static;
use std::collections::HashMap;

// CodeUnknown is for errors of unknown reason.
pub const CODE_UNKNOWN: isize = -1;
// CodeExecResultIsEmpty indicates execution result is empty.
pub const CODE_EXEC_RESULT_IS_EMPTY: isize = 3;

// Expression error codes.

// CodeMissConnectionID indicates connection id is missing.
pub const CODE_MISS_CONNECTION_ID: isize = 1;

// Special error codes.

// CodeResultUndetermined indicates the sql execution result is undetermined.
pub const CODE_RESULT_UNDETER_MINED: isize = 2;

#[derive(Debug)]
pub struct ErrClass {
    pub code: isize,
    pub name: String,
}

// impl ErrClass {
//     pub fn new_std_err(&self, code: isize, message: &ErrMessage) -> ErrClass {}
//
//     pub fn new_std(&self, code: isize) -> ErrClass {
//         let msg = mysql_err_name.get(&code).unwrap();
//         self.new_std_err(code, msg)
//     }
// }

lazy_static! {
    pub static ref class_autoid: ErrClass = ErrClass {
        code: 1,
        name: "autoid".to_string()
    };
    pub static ref class_ddl: ErrClass = ErrClass {
        code: 2,
        name: "ddl".to_string()
    };
    pub static ref class_domain: ErrClass = ErrClass {
        code: 3,
        name: "domain".to_string()
    };
    pub static ref class_evaluator: ErrClass = ErrClass {
        code: 4,
        name: "evaluator".to_string()
    };
    pub static ref class_executor: ErrClass = ErrClass {
        code: 5,
        name: "executor".to_string()
    };
    pub static ref class_expression: ErrClass = ErrClass {
        code: 6,
        name: "expression".to_string()
    };
    pub static ref class_admin: ErrClass = ErrClass {
        code: 7,
        name: "admin".to_string()
    };
    pub static ref class_kv: ErrClass = ErrClass {
        code: 8,
        name: "kv".to_string()
    };
    pub static ref class_meta: ErrClass = ErrClass {
        code: 9,
        name: "meta".to_string()
    };
    pub static ref class_optimizer: ErrClass = ErrClass {
        code: 10,
        name: "planner".to_string()
    };
    pub static ref class_parser: ErrClass = ErrClass {
        code: 11,
        name: "parser".to_string()
    };
    pub static ref class_perf_schema: ErrClass = ErrClass {
        code: 12,
        name: "perfschema".to_string()
    };
    pub static ref class_privilege: ErrClass = ErrClass {
        code: 13,
        name: "privilege".to_string()
    };
    pub static ref class_schema: ErrClass = ErrClass {
        code: 14,
        name: "schema".to_string()
    };
    pub static ref class_server: ErrClass = ErrClass {
        code: 15,
        name: "server".to_string()
    };
    pub static ref class_structure: ErrClass = ErrClass {
        code: 16,
        name: "structure".to_string()
    };
    pub static ref class_variable: ErrClass = ErrClass {
        code: 17,
        name: "variable".to_string()
    };
    pub static ref class_xeval: ErrClass = ErrClass {
        code: 18,
        name: "xeval".to_string()
    };
    pub static ref class_table: ErrClass = ErrClass {
        code: 19,
        name: "table".to_string()
    };
    pub static ref class_types: ErrClass = ErrClass {
        code: 20,
        name: "types".to_string()
    };
    pub static ref class_global: ErrClass = ErrClass {
        code: 21,
        name: "global".to_string()
    };
    pub static ref class_mock_tikv: ErrClass = ErrClass {
        code: 22,
        name: "mocktikv".to_string()
    };
    pub static ref class_json: ErrClass = ErrClass {
        code: 23,
        name: "json".to_string()
    };
    pub static ref class_tikv: ErrClass = ErrClass {
        code: 24,
        name: "tikv".to_string()
    };
    pub static ref class_session: ErrClass = ErrClass {
        code: 25,
        name: "session".to_string()
    };
    pub static ref class_plugin: ErrClass = ErrClass {
        code: 26,
        name: "plugin".to_string()
    };
    pub static ref class_util: ErrClass = ErrClass {
        code: 27,
        name: "util".to_string()
    };

    // RegisterErrorClass registers new error class for terror.
    pub static ref err_class_2_desc: HashMap<isize, &'static ErrClass> = {
        let mut m = HashMap::<isize, &'static ErrClass>::new();
        m.insert(class_autoid.code, &class_autoid);
        m.insert(class_ddl.code, &class_ddl);
        m.insert(class_domain.code, &class_domain);
        m.insert(class_evaluator.code, &class_evaluator);
        m.insert(class_executor.code, &class_executor);
        m.insert(class_expression.code, &class_expression);
        m.insert(class_admin.code, &class_admin);
        m.insert(class_kv.code, &class_kv);
        m.insert(class_meta.code, &class_meta);
        m.insert(class_optimizer.code, &class_optimizer);
        m.insert(class_parser.code, &class_parser);
        m.insert(class_perf_schema.code, &class_perf_schema);
        m.insert(class_privilege.code, &class_privilege);
        m.insert(class_schema.code, &class_schema);
        m.insert(class_server.code, &class_server);
        m.insert(class_structure.code, &class_structure);
        m.insert(class_variable.code, &class_variable);
        m.insert(class_xeval.code, &class_xeval);
        m.insert(class_table.code, &class_table);
        m.insert(class_types.code, &class_types);
        m.insert(class_global.code, &class_global);
        m.insert(class_mock_tikv.code, &class_mock_tikv);
        m.insert(class_json.code, &class_json);
        m.insert(class_tikv.code, &class_tikv);
        m.insert(class_session.code, &class_session);
        m.insert(class_plugin.code, &class_plugin);
        m.insert(class_util.code, &class_util);

        m
    };
}
