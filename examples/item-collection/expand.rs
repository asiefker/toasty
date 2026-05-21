#![feature(prelude_import)]
//! Three-level item collection: Tenant -> User -> Todo.
//!
//! All three models share a single DynamoDB table. The partition key is
//! `tenant_id`; the sort key is composed by the driver from `__model` plus
//! each model's own local PK fields:
//!
//!   Tenant row: __sk = "Tenant#"
//!   User row:   __sk = "User#<user_id>#"
//!   Todo row:   __sk = "Todo#<user_id>#<todo_id>#"
//!
//! Querying `tenant.users()` becomes `begins_with(__sk, "User#")` and
//! `user.todos()` becomes `begins_with(__sk, "Todo#<user_id>#")` — both
//! single-partition queries with no scan.
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use uuid::Uuid;
struct Tenant {
    #[key]
    #[auto]
    id: uuid::Uuid,
    name: String,
    #[has_many]
    users: toasty::HasMany<User>,
}
#[automatically_derived]
impl ::core::fmt::Debug for Tenant {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "Tenant",
            "id",
            &self.id,
            "name",
            &self.name,
            "users",
            &&self.users,
        )
    }
}
const _: () = {
    use toasty as _toasty;
    impl Tenant {
        fn fields() -> TenantFields<Tenant> {
            TenantFields {
                path: _toasty::codegen_support::Path::root(),
            }
        }
        async fn get_by_id(
            executor: &mut dyn _toasty::codegen_support::Executor,
            id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> _toasty::codegen_support::Result<Tenant> {
            Self::filter_by_id(id).get(executor).await
        }
        fn update_by_id(
            id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> TenantUpdate {
            Self::filter_by_id(id).update()
        }
        async fn delete_by_id(
            executor: &mut dyn _toasty::codegen_support::Executor,
            id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> _toasty::codegen_support::Result<()> {
            Self::filter_by_id(id).delete().exec(executor).await
        }
        fn filter_by_id(
            id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> TenantQuery {
            TenantQuery::default().filter_by_id(id)
        }
        fn users(
            &self,
        ) -> <toasty::HasMany<User> as _toasty::codegen_support::Relation>::Many {
            if false {
                let _ = &self.users;
            }
            #[allow(unreachable_code)]
            if false {
                fn load<__toasty_T: _toasty::codegen_support::Model>() -> __toasty_T {
                    __toasty_T::load(::core::panicking::panic("not yet implemented"))
                        .unwrap()
                }
                #[diagnostic::on_unimplemented(
                    message = "HasMany requires the {__toasty_A}::tenant field to be of type `BelongsTo<Self>`, but it was `{Self}` instead",
                    label = "Has many associations require the target to include a back-reference",
                    note = "Note 1",
                )]
                trait Verify<__toasty_A> {}
                #[diagnostic::do_not_recommend]
                impl<__toasty_A> Verify<__toasty_A>
                for _toasty::codegen_support::BelongsTo<Tenant> {}
                #[diagnostic::do_not_recommend]
                impl<__toasty_A> Verify<__toasty_A>
                for _toasty::codegen_support::BelongsTo<Option<Tenant>> {}
                fn verify<__toasty_T: Verify<__toasty_A>, __toasty_A>(_: &__toasty_T) {}
                let instance = load::<
                    <toasty::HasMany<User> as _toasty::codegen_support::Relation>::Model,
                >();
                verify::<
                    _,
                    <toasty::HasMany<User> as _toasty::codegen_support::Relation>::Model,
                >(instance.verify_pair_belongs_to_exists_for_tenant());
            }
            {
                use _toasty::codegen_support::IntoStatement;
                <toasty::HasMany<
                    User,
                > as _toasty::codegen_support::Relation>::Many::from_stmt(
                    _toasty::codegen_support::stmt::Association::many(
                        self.into_statement().into_query().unwrap().to_list(),
                        Self::fields().users().into(),
                    ),
                )
            }
        }
        #[doc(hidden)]
        pub const fn __check_create_fields(__provided: &[&str]) {
            if !<String as _toasty::codegen_support::Field>::NULLABLE
                && !_toasty::codegen_support::const_contains(__provided, "name")
            {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "missing required field `name` in create! for `Tenant`",
                        ),
                    );
                };
            }
        }
        fn create() -> TenantCreate {
            TenantCreate::default()
        }
        fn create_many() -> _toasty::codegen_support::stmt::CreateMany<Tenant> {
            _toasty::codegen_support::stmt::CreateMany::default()
        }
        fn update(&mut self) -> TenantUpdate<&mut Self> {
            let mut s = TenantUpdate {
                assignments: _toasty::codegen_support::core::stmt::Assignments::default(),
                condition: None,
                target: self,
            };
            s.apply_update_defaults();
            s
        }
        fn all() -> TenantQuery {
            TenantQuery::default()
        }
        fn filter(expr: _toasty::codegen_support::stmt::Expr<bool>) -> TenantQuery {
            TenantQuery::from_stmt(_toasty::codegen_support::stmt::Query::filter(expr))
        }
        fn delete(self) -> _toasty::codegen_support::stmt::Delete<()> {
            {
                let __delete = TenantQuery::default().filter_by_id(&self.id).delete();
                __delete
            }
        }
    }
    impl _toasty::codegen_support::Register for Tenant {
        fn id() -> _toasty::codegen_support::core::schema::app::ModelId {
            static ID: std::sync::OnceLock<
                _toasty::codegen_support::core::schema::app::ModelId,
            > = std::sync::OnceLock::new();
            *ID.get_or_init(|| _toasty::codegen_support::generate_unique_id())
        }
        fn schema() -> _toasty::codegen_support::core::schema::app::Model {
            let id = Tenant::id();
            _toasty::codegen_support::core::schema::app::Model::Root(_toasty::codegen_support::core::schema::app::ModelRoot {
                id,
                name: _toasty::codegen_support::core::schema::Name {
                    parts: ::alloc::boxed::box_assume_init_into_vec_unsafe(
                        ::alloc::intrinsics::write_box_via_move(
                            ::alloc::boxed::Box::new_uninit(),
                            ["tenant".to_string()],
                        ),
                    ),
                },
                fields: ::alloc::boxed::box_assume_init_into_vec_unsafe(
                    ::alloc::intrinsics::write_box_via_move(
                        ::alloc::boxed::Box::new_uninit(),
                        [
                            _toasty::codegen_support::core::schema::app::Field {
                                id: _toasty::codegen_support::core::schema::app::FieldId {
                                    model: Tenant::id(),
                                    index: 0,
                                },
                                name: _toasty::codegen_support::core::schema::app::FieldName {
                                    app: Some("id".to_string()),
                                    storage: None,
                                },
                                ty: <uuid::Uuid as _toasty::codegen_support::Field>::field_ty(
                                    None,
                                ),
                                nullable: <uuid::Uuid as _toasty::codegen_support::Field>::NULLABLE,
                                primary_key: true,
                                auto: Some(
                                    <uuid::Uuid as _toasty::codegen_support::Auto>::STRATEGY,
                                ),
                                versionable: false,
                                deferred: false,
                                constraints: ::alloc::vec::Vec::new(),
                                variant: None,
                            },
                            _toasty::codegen_support::core::schema::app::Field {
                                id: _toasty::codegen_support::core::schema::app::FieldId {
                                    model: Tenant::id(),
                                    index: 1,
                                },
                                name: _toasty::codegen_support::core::schema::app::FieldName {
                                    app: Some("name".to_string()),
                                    storage: None,
                                },
                                ty: <String as _toasty::codegen_support::Field>::field_ty(
                                    None,
                                ),
                                nullable: <String as _toasty::codegen_support::Field>::NULLABLE,
                                primary_key: false,
                                auto: None,
                                versionable: false,
                                deferred: false,
                                constraints: ::alloc::vec::Vec::new(),
                                variant: None,
                            },
                            _toasty::codegen_support::core::schema::app::Field {
                                id: _toasty::codegen_support::core::schema::app::FieldId {
                                    model: Tenant::id(),
                                    index: 2,
                                },
                                name: _toasty::codegen_support::core::schema::app::FieldName {
                                    app: Some("users".to_string()),
                                    storage: None,
                                },
                                ty: <toasty::HasMany<
                                    User,
                                > as _toasty::codegen_support::Relation>::has_many_field_ty(
                                    _toasty::codegen_support::core::schema::Name {
                                        parts: ::alloc::boxed::box_assume_init_into_vec_unsafe(
                                            ::alloc::intrinsics::write_box_via_move(
                                                ::alloc::boxed::Box::new_uninit(),
                                                ["user".to_string()],
                                            ),
                                        ),
                                    },
                                    None,
                                    None,
                                ),
                                nullable: <toasty::HasMany<
                                    User,
                                > as _toasty::codegen_support::Relation>::nullable(),
                                primary_key: false,
                                auto: None,
                                versionable: false,
                                deferred: false,
                                constraints: ::alloc::vec::Vec::new(),
                                variant: None,
                            },
                        ],
                    ),
                ),
                primary_key: _toasty::codegen_support::core::schema::app::PrimaryKey {
                    fields: ::alloc::boxed::box_assume_init_into_vec_unsafe(
                        ::alloc::intrinsics::write_box_via_move(
                            ::alloc::boxed::Box::new_uninit(),
                            [
                                _toasty::codegen_support::core::schema::app::FieldId {
                                    model: id,
                                    index: 0,
                                },
                            ],
                        ),
                    ),
                    index: _toasty::codegen_support::core::schema::app::IndexId {
                        model: id,
                        index: 0,
                    },
                },
                table_name: None,
                item_collection: None,
                indices: ::alloc::boxed::box_assume_init_into_vec_unsafe(
                    ::alloc::intrinsics::write_box_via_move(
                        ::alloc::boxed::Box::new_uninit(),
                        [
                            _toasty::codegen_support::core::schema::app::Index {
                                id: _toasty::codegen_support::core::schema::app::IndexId {
                                    model: id,
                                    index: 0,
                                },
                                name: None,
                                fields: ::alloc::boxed::box_assume_init_into_vec_unsafe(
                                    ::alloc::intrinsics::write_box_via_move(
                                        ::alloc::boxed::Box::new_uninit(),
                                        [
                                            _toasty::codegen_support::core::schema::app::IndexField {
                                                field: _toasty::codegen_support::core::schema::app::FieldId {
                                                    model: id,
                                                    index: 0,
                                                },
                                                op: _toasty::codegen_support::core::schema::db::IndexOp::Eq,
                                                scope: _toasty::codegen_support::core::schema::db::IndexScope::Partition,
                                            },
                                        ],
                                    ),
                                ),
                                unique: true,
                                primary_key: true,
                            },
                        ],
                    ),
                ),
                version_field: None,
            })
        }
        fn register(
            model_set: &mut _toasty::codegen_support::core::schema::app::ModelSet,
        ) {
            if model_set.contains(Self::id()) {
                return;
            }
            model_set.add(Self::schema());
            <uuid::Uuid as _toasty::codegen_support::Field>::register(model_set);
            <String as _toasty::codegen_support::Field>::register(model_set);
            <<toasty::HasMany<
                User,
            > as _toasty::codegen_support::Relation>::Model as _toasty::codegen_support::Register>::register(
                model_set,
            );
        }
    }
    #[allow(non_upper_case_globals)]
    const _: () = {
        static __INVENTORY: ::inventory::Node = ::inventory::Node {
            value: &{
                _toasty::codegen_support::DiscoverItem::new(
                    "example-item-collection",
                    |model_set| {
                        <Tenant as _toasty::codegen_support::Register>::register(
                            model_set,
                        );
                    },
                )
            },
            next: ::inventory::__private::UnsafeCell::new(
                ::inventory::__private::Option::None,
            ),
        };
        #[link_section = ".text.startup"]
        unsafe extern "C" fn __ctor() {
            unsafe { ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY) }
        }
        #[used]
        #[link_section = ".init_array"]
        static __CTOR: unsafe extern "C" fn() = __ctor;
    };
    impl _toasty::codegen_support::Load for Tenant {
        type Output = Self;
        fn ty() -> _toasty::codegen_support::core::stmt::Type {
            _toasty::codegen_support::core::stmt::Type::Model(
                <Self as _toasty::codegen_support::Register>::id(),
            )
        }
        fn load(
            value: _toasty::codegen_support::core::stmt::Value,
        ) -> _toasty::codegen_support::Result<Self> {
            match value {
                _toasty::codegen_support::core::stmt::Value::Null => {
                    Err(_toasty::codegen_support::Error::record_not_found("Tenant"))
                }
                _toasty::codegen_support::core::stmt::Value::Record(mut record) => {
                    Ok(Tenant {
                        id: <uuid::Uuid as _toasty::codegen_support::Load>::load(
                            record[0].take(),
                        )?,
                        name: <String as _toasty::codegen_support::Load>::load(
                            record[1].take(),
                        )?,
                        users: _toasty::codegen_support::HasMany::load(
                            record[2usize].take(),
                        )?,
                    })
                }
                value => {
                    Err(
                        _toasty::codegen_support::Error::type_conversion(value, "Tenant"),
                    )
                }
            }
        }
        fn reload(
            target: &mut Self,
            value: _toasty::codegen_support::core::stmt::Value,
        ) -> _toasty::codegen_support::Result<()> {
            for (field, value) in value.into_sparse_record().into_iter() {
                match field {
                    0 => {
                        <uuid::Uuid as _toasty::codegen_support::Load>::reload(
                            &mut target.id,
                            value,
                        )?
                    }
                    1 => {
                        <String as _toasty::codegen_support::Load>::reload(
                            &mut target.name,
                            value,
                        )?
                    }
                    2 => target.users.unload(),
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "not yet implemented: {0}",
                                format_args!(
                                    "handle unknown field id in reload after update",
                                ),
                            ),
                        );
                    }
                }
            }
            Ok(())
        }
    }
    impl _toasty::codegen_support::Model for Tenant {
        type Query = TenantQuery;
        type Create = TenantCreate;
        type Update<'a> = TenantUpdate<&'a mut Self>;
        type UpdateQuery = TenantUpdate;
        type Path<__Origin> = TenantFields<__Origin>;
        type PrimaryKey = uuid::Uuid;
        const CREATE_META: _toasty::codegen_support::CreateMeta = _toasty::codegen_support::CreateMeta {
            fields: &[
                _toasty::codegen_support::CreateField {
                    name: "name",
                    required: !<String as _toasty::codegen_support::Field>::NULLABLE,
                },
            ],
            model_name: "Tenant",
        };
        fn new_path<__Origin>(
            path: _toasty::codegen_support::Path<__Origin, Self>,
        ) -> Self::Path<__Origin> {
            TenantFields::from_path(path)
        }
        fn find_by_primary_key(
            id: _toasty::codegen_support::stmt::Expr<Self::PrimaryKey>,
        ) -> Self::Query {
            Self::filter_by_id(id)
        }
    }
    impl _toasty::codegen_support::Relation for Tenant {
        type Model = Tenant;
        type Expr = Tenant;
        type Query = TenantQuery;
        type Create = TenantCreate;
        type Many = Many;
        type ManyField<__Origin> = TenantListFields<__Origin>;
        type One = One;
        type OneField<__Origin> = TenantFields<__Origin>;
        type OptionOne = OptionOne;
        fn new_many_field<__Origin>(
            path: _toasty::codegen_support::Path<
                __Origin,
                _toasty::codegen_support::List<Self::Model>,
            >,
        ) -> TenantListFields<__Origin> {
            TenantListFields::from_path(path)
        }
        fn field_name_to_id(
            name: &str,
        ) -> _toasty::codegen_support::core::schema::app::FieldId {
            use _toasty::codegen_support::{Model, Register};
            match name {
                "id" => {
                    _toasty::codegen_support::core::schema::app::FieldId {
                        model: Self::id(),
                        index: 0,
                    }
                }
                "name" => {
                    _toasty::codegen_support::core::schema::app::FieldId {
                        model: Self::id(),
                        index: 1,
                    }
                }
                "users" => {
                    _toasty::codegen_support::core::schema::app::FieldId {
                        model: Self::id(),
                        index: 2,
                    }
                }
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "not yet implemented: {0}",
                            format_args!("field_name_to_id: {0}", name),
                        ),
                    );
                }
            }
        }
    }
    impl _toasty::codegen_support::stmt::IntoExpr<Tenant> for Tenant {
        fn into_expr(self) -> _toasty::codegen_support::stmt::Expr<Tenant> {
            let expr: _toasty::codegen_support::stmt::Expr<uuid::Uuid> = _toasty::codegen_support::IntoExpr::into_expr(
                self.id,
            );
            expr.cast()
        }
        fn by_ref(&self) -> _toasty::codegen_support::stmt::Expr<Tenant> {
            let expr: _toasty::codegen_support::stmt::Expr<uuid::Uuid> = _toasty::codegen_support::IntoExpr::into_expr(
                (&self.id),
            );
            expr.cast()
        }
    }
    impl _toasty::codegen_support::Assign<Tenant> for Tenant {
        fn into_assignment(self) -> _toasty::codegen_support::stmt::Assignment<Tenant> {
            _toasty::codegen_support::stmt::set(
                <Self as _toasty::codegen_support::IntoExpr<Tenant>>::into_expr(self),
            )
        }
    }
    impl _toasty::codegen_support::IntoStatement for &Tenant {
        type Returning = Tenant;
        fn into_statement(self) -> _toasty::codegen_support::Statement<Tenant> {
            use _toasty::codegen_support::IntoStatement;
            _toasty::codegen_support::IntoStatement::into_statement(
                TenantQuery::default().filter_by_id(&self.id).one(),
            )
        }
    }
    impl _toasty::codegen_support::IntoStatement for &mut Tenant {
        type Returning = Tenant;
        fn into_statement(self) -> _toasty::codegen_support::Statement<Tenant> {
            (&*self).into_statement()
        }
    }
    impl _toasty::codegen_support::IntoStatement for Tenant {
        type Returning = Tenant;
        fn into_statement(self) -> _toasty::codegen_support::Statement<Tenant> {
            (&self).into_statement()
        }
    }
    struct TenantFields<__Origin> {
        path: _toasty::codegen_support::Path<__Origin, Tenant>,
    }
    impl<__Origin> TenantFields<__Origin> {
        const fn from_path(
            path: _toasty::codegen_support::Path<__Origin, Tenant>,
        ) -> TenantFields<__Origin> {
            TenantFields { path }
        }
        fn path(&self) -> _toasty::codegen_support::Path<__Origin, Tenant> {
            self.path.clone()
        }
        fn eq(
            self,
            rhs: impl _toasty::codegen_support::IntoExpr<Tenant>,
        ) -> _toasty::codegen_support::stmt::Expr<bool> {
            use _toasty::codegen_support::IntoExpr;
            self.path.eq(rhs.into_expr())
        }
        fn in_query(
            self,
            rhs: impl _toasty::codegen_support::IntoStatement<
                Returning = _toasty::codegen_support::List<Tenant>,
            >,
        ) -> _toasty::codegen_support::stmt::Expr<bool> {
            self.path.in_query(rhs)
        }
        fn create(&self) -> TenantCreate {
            TenantCreate::default()
        }
        fn id(&self) -> <uuid::Uuid as _toasty::codegen_support::Field>::Path<__Origin> {
            <uuid::Uuid as _toasty::codegen_support::Field>::new_path(
                self
                    .path()
                    .chain(
                        _toasty::codegen_support::Path::<
                            Tenant,
                            <uuid::Uuid as _toasty::codegen_support::Field>::ExprTarget,
                        >::from_field_index(0),
                    ),
            )
        }
        fn name(&self) -> <String as _toasty::codegen_support::Field>::Path<__Origin> {
            <String as _toasty::codegen_support::Field>::new_path(
                self
                    .path()
                    .chain(
                        _toasty::codegen_support::Path::<
                            Tenant,
                            <String as _toasty::codegen_support::Field>::ExprTarget,
                        >::from_field_index(1),
                    ),
            )
        }
        fn users(
            &self,
        ) -> <toasty::HasMany<
            User,
        > as _toasty::codegen_support::Relation>::ManyField<__Origin> {
            <toasty::HasMany<
                User,
            > as _toasty::codegen_support::Relation>::ManyField::from_path(
                self
                    .path()
                    .chain(
                        _toasty::codegen_support::Path::<Tenant, _>::from_field_index(2),
                    ),
            )
        }
    }
    impl<__Origin> Into<_toasty::codegen_support::Path<__Origin, Tenant>>
    for TenantFields<__Origin> {
        fn into(self) -> _toasty::codegen_support::Path<__Origin, Tenant> {
            self.path
        }
    }
    impl<__Origin> _toasty::codegen_support::IntoExpr<Tenant>
    for TenantFields<__Origin> {
        fn into_expr(self) -> _toasty::codegen_support::stmt::Expr<Tenant> {
            self.path.into_expr()
        }
        fn by_ref(&self) -> _toasty::codegen_support::stmt::Expr<Tenant> {
            self.path.by_ref()
        }
    }
    struct TenantListFields<__Origin> {
        path: _toasty::codegen_support::Path<
            __Origin,
            _toasty::codegen_support::List<Tenant>,
        >,
    }
    impl<__Origin> TenantListFields<__Origin> {
        const fn from_path(
            path: _toasty::codegen_support::Path<
                __Origin,
                _toasty::codegen_support::List<Tenant>,
            >,
        ) -> TenantListFields<__Origin> {
            TenantListFields { path }
        }
        fn path(
            &self,
        ) -> _toasty::codegen_support::Path<
            __Origin,
            _toasty::codegen_support::List<Tenant>,
        > {
            self.path.clone()
        }
        /// Filter the parent model by a condition on the associated
        /// (child) model. Returns `true` when **any** associated record
        /// satisfies `filter`.
        fn any(
            self,
            filter: _toasty::codegen_support::stmt::Expr<bool>,
        ) -> _toasty::codegen_support::stmt::Expr<bool> {
            self.path.any(filter)
        }
        /// Filter the parent model by a condition on the associated
        /// (child) model. Returns `true` when **all** associated records
        /// satisfy `filter` (vacuously true when there are no
        /// associated records).
        fn all(
            self,
            filter: _toasty::codegen_support::stmt::Expr<bool>,
        ) -> _toasty::codegen_support::stmt::Expr<bool> {
            self.path.all(filter)
        }
        fn create(&self) -> TenantCreate {
            TenantCreate::default()
        }
        fn id(
            &self,
        ) -> <uuid::Uuid as _toasty::codegen_support::Field>::ListPath<__Origin> {
            <uuid::Uuid as _toasty::codegen_support::Field>::new_list_path(
                self
                    .path()
                    .chain(
                        _toasty::codegen_support::Path::<Tenant, _>::from_field_index(0),
                    ),
            )
        }
        fn name(
            &self,
        ) -> <String as _toasty::codegen_support::Field>::ListPath<__Origin> {
            <String as _toasty::codegen_support::Field>::new_list_path(
                self
                    .path()
                    .chain(
                        _toasty::codegen_support::Path::<Tenant, _>::from_field_index(1),
                    ),
            )
        }
        fn users(
            &self,
        ) -> <toasty::HasMany<
            User,
        > as _toasty::codegen_support::Relation>::ManyField<__Origin> {
            <toasty::HasMany<
                User,
            > as _toasty::codegen_support::Relation>::ManyField::from_path(
                self
                    .path()
                    .chain(
                        _toasty::codegen_support::Path::<Tenant, _>::from_field_index(2),
                    ),
            )
        }
    }
    impl<
        __Origin,
    > Into<
        _toasty::codegen_support::Path<__Origin, _toasty::codegen_support::List<Tenant>>,
    > for TenantListFields<__Origin> {
        fn into(
            self,
        ) -> _toasty::codegen_support::Path<
            __Origin,
            _toasty::codegen_support::List<Tenant>,
        > {
            self.path
        }
    }
    impl<
        __Origin,
    > _toasty::codegen_support::IntoExpr<_toasty::codegen_support::List<Tenant>>
    for TenantListFields<__Origin> {
        fn into_expr(
            self,
        ) -> _toasty::codegen_support::stmt::Expr<
            _toasty::codegen_support::List<Tenant>,
        > {
            self.path.into_expr()
        }
        fn by_ref(
            &self,
        ) -> _toasty::codegen_support::stmt::Expr<
            _toasty::codegen_support::List<Tenant>,
        > {
            self.path.by_ref()
        }
    }
    struct TenantQuery {
        stmt: _toasty::codegen_support::stmt::Query<
            _toasty::codegen_support::List<Tenant>,
        >,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TenantQuery {
        #[inline]
        fn clone(&self) -> TenantQuery {
            TenantQuery {
                stmt: ::core::clone::Clone::clone(&self.stmt),
            }
        }
    }
    impl TenantQuery {
        const fn from_stmt(
            stmt: _toasty::codegen_support::stmt::Query<
                _toasty::codegen_support::List<Tenant>,
            >,
        ) -> TenantQuery {
            TenantQuery { stmt }
        }
        async fn get_by_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> _toasty::codegen_support::Result<Tenant> {
            self.filter_by_id(id).get(executor).await
        }
        fn update_by_id(
            self,
            id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> TenantUpdate {
            self.filter_by_id(id).update()
        }
        async fn delete_by_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> _toasty::codegen_support::Result<()> {
            self.filter_by_id(id).delete().exec(executor).await
        }
        fn filter_by_id(
            self,
            id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> TenantQuery {
            self.filter(Tenant::fields().id().eq(id))
        }
        async fn exec(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
        ) -> _toasty::codegen_support::Result<Vec<Tenant>> {
            executor.exec(self.stmt.into()).await
        }
        fn first(self) -> _toasty::codegen_support::stmt::Query<Option<Tenant>> {
            self.stmt.first()
        }
        fn one(self) -> _toasty::codegen_support::stmt::Query<Tenant> {
            self.stmt.one()
        }
        async fn get(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
        ) -> _toasty::codegen_support::Result<Tenant> {
            self.one().exec(executor).await
        }
        fn update(self) -> TenantUpdate {
            TenantUpdate::from(self)
        }
        fn count(self) -> _toasty::codegen_support::stmt::Query<u64> {
            self.stmt.count()
        }
        fn select<__E, __T>(
            self,
            projection: __E,
        ) -> _toasty::codegen_support::stmt::Query<_toasty::codegen_support::List<__T>>
        where
            __E: _toasty::codegen_support::IntoExpr<__T>,
            __T: _toasty::codegen_support::Load,
        {
            self.stmt.select(projection)
        }
        fn delete(self) -> _toasty::codegen_support::stmt::Delete<()> {
            self.stmt.delete()
        }
        fn paginate(
            self,
            per_page: usize,
        ) -> _toasty::codegen_support::stmt::Paginate<Tenant> {
            _toasty::codegen_support::stmt::Paginate::new(self.stmt, per_page)
        }
        fn filter(
            self,
            expr: _toasty::codegen_support::stmt::Expr<bool>,
        ) -> TenantQuery {
            TenantQuery {
                stmt: self.stmt.and(expr),
            }
        }
        fn order_by(
            mut self,
            order_by: impl Into<_toasty::codegen_support::stmt::OrderBy>,
        ) -> TenantQuery {
            self.stmt.order_by(order_by);
            self
        }
        fn latest_by<__toasty_T>(
            mut self,
            field: _toasty::codegen_support::stmt::Path<Tenant, __toasty_T>,
        ) -> TenantQuery {
            self.stmt.latest_by(field);
            self
        }
        fn limit(mut self, n: usize) -> TenantQuery {
            self.stmt.limit(n);
            self
        }
        fn offset(mut self, n: usize) -> TenantQuery {
            self.stmt.offset(n);
            self
        }
        fn include<__toasty_T>(
            mut self,
            path: impl _toasty::codegen_support::Into<
                _toasty::codegen_support::Path<Tenant, __toasty_T>,
            >,
        ) -> TenantQuery {
            self.stmt.include(path.into());
            self
        }
        fn users(
            mut self,
        ) -> <toasty::HasMany<User> as _toasty::codegen_support::Relation>::Query {
            use _toasty::codegen_support::IntoStatement;
            <toasty::HasMany<
                User,
            > as _toasty::codegen_support::Relation>::Query::from_stmt(
                _toasty::codegen_support::stmt::Association::many(
                        self.stmt,
                        Tenant::fields().users().into(),
                    )
                    .into_statement()
                    .into_query()
                    .unwrap(),
            )
        }
    }
    impl _toasty::codegen_support::IntoStatement for TenantQuery {
        type Returning = _toasty::codegen_support::List<Tenant>;
        fn into_statement(
            self,
        ) -> _toasty::codegen_support::Statement<
            _toasty::codegen_support::List<Tenant>,
        > {
            use _toasty::codegen_support::IntoStatement;
            self.stmt.into_statement()
        }
    }
    impl _toasty::codegen_support::IntoStatement for &TenantQuery {
        type Returning = _toasty::codegen_support::List<Tenant>;
        fn into_statement(
            self,
        ) -> _toasty::codegen_support::Statement<
            _toasty::codegen_support::List<Tenant>,
        > {
            use _toasty::codegen_support::IntoStatement;
            self.stmt.clone().into_statement()
        }
    }
    impl _toasty::codegen_support::stmt::IntoScope<Tenant> for TenantQuery {
        fn into_scope(
            self,
        ) -> _toasty::codegen_support::Statement<
            _toasty::codegen_support::List<Tenant>,
        > {
            use _toasty::codegen_support::stmt::IntoScope;
            self.stmt.into_scope()
        }
    }
    impl _toasty::codegen_support::stmt::IntoScope<Tenant> for &TenantQuery {
        fn into_scope(
            self,
        ) -> _toasty::codegen_support::Statement<
            _toasty::codegen_support::List<Tenant>,
        > {
            use _toasty::codegen_support::stmt::IntoScope;
            self.stmt.clone().into_scope()
        }
    }
    impl _toasty::codegen_support::Default for TenantQuery {
        fn default() -> TenantQuery {
            TenantQuery {
                stmt: _toasty::codegen_support::stmt::Query::all(),
            }
        }
    }
    struct TenantCreate {
        stmt: _toasty::codegen_support::stmt::Insert<Tenant>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TenantCreate {
        #[inline]
        fn clone(&self) -> TenantCreate {
            TenantCreate {
                stmt: ::core::clone::Clone::clone(&self.stmt),
            }
        }
    }
    impl TenantCreate {
        fn id(
            mut self,
            id: impl _toasty::codegen_support::IntoExpr<
                <uuid::Uuid as _toasty::codegen_support::Field>::ExprTarget,
            >,
        ) -> Self {
            self.stmt.set(0, id.into_expr());
            self
        }
        fn name(
            mut self,
            name: impl _toasty::codegen_support::IntoExpr<
                <String as _toasty::codegen_support::Field>::ExprTarget,
            >,
        ) -> Self {
            self.stmt.set(1, name.into_expr());
            self
        }
        fn user(
            mut self,
            user: impl _toasty::codegen_support::IntoExpr<
                <toasty::HasMany<User> as _toasty::codegen_support::Relation>::Expr,
            >,
        ) -> Self {
            self.stmt.insert(2, user.into_expr());
            self
        }
        fn users(
            mut self,
            users: impl _toasty::codegen_support::IntoExpr<
                _toasty::codegen_support::List<
                    <toasty::HasMany<User> as _toasty::codegen_support::Relation>::Model,
                >,
            >,
        ) -> Self {
            self.stmt.insert_all(2, users.into_expr());
            self
        }
        async fn exec(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
        ) -> _toasty::codegen_support::Result<Tenant> {
            executor.exec(self.stmt.into()).await
        }
    }
    impl _toasty::codegen_support::IntoInsert for TenantCreate {
        type Model = Tenant;
        fn into_insert(self) -> _toasty::codegen_support::stmt::Insert<Tenant> {
            self.stmt
        }
    }
    impl _toasty::codegen_support::IntoStatement for TenantCreate {
        type Returning = Tenant;
        fn into_statement(self) -> _toasty::codegen_support::Statement<Tenant> {
            self.stmt.into()
        }
    }
    impl _toasty::codegen_support::IntoExpr<Tenant> for TenantCreate {
        fn into_expr(self) -> _toasty::codegen_support::stmt::Expr<Tenant> {
            self.stmt.into()
        }
        fn by_ref(&self) -> _toasty::codegen_support::stmt::Expr<Tenant> {
            ::core::panicking::panic("not yet implemented")
        }
    }
    impl _toasty::codegen_support::IntoExpr<Option<Tenant>> for TenantCreate {
        fn into_expr(self) -> _toasty::codegen_support::stmt::Expr<Option<Tenant>> {
            self.stmt.into()
        }
        fn by_ref(&self) -> _toasty::codegen_support::stmt::Expr<Option<Tenant>> {
            ::core::panicking::panic("not yet implemented")
        }
    }
    impl _toasty::codegen_support::Assign<Tenant> for TenantCreate {
        fn into_assignment(self) -> _toasty::codegen_support::stmt::Assignment<Tenant> {
            _toasty::codegen_support::stmt::set(
                <Self as _toasty::codegen_support::IntoExpr<Tenant>>::into_expr(self),
            )
        }
    }
    impl _toasty::codegen_support::Assign<Option<Tenant>> for TenantCreate {
        fn into_assignment(
            self,
        ) -> _toasty::codegen_support::stmt::Assignment<Option<Tenant>> {
            _toasty::codegen_support::stmt::set(
                <Self as _toasty::codegen_support::IntoExpr<
                    Option<Tenant>,
                >>::into_expr(self),
            )
        }
    }
    impl Default for TenantCreate {
        fn default() -> TenantCreate {
            let mut s = TenantCreate {
                stmt: _toasty::codegen_support::stmt::Insert::blank_single(),
            };
            s
        }
    }
    struct TenantUpdate<
        __toasty_T: _toasty::codegen_support::UpdateTarget = TenantQuery,
    > {
        assignments: _toasty::codegen_support::core::stmt::Assignments,
        condition: Option<_toasty::codegen_support::core::stmt::Expr>,
        target: __toasty_T,
    }
    #[automatically_derived]
    impl<
        __toasty_T: ::core::clone::Clone + _toasty::codegen_support::UpdateTarget,
    > ::core::clone::Clone for TenantUpdate<__toasty_T> {
        #[inline]
        fn clone(&self) -> TenantUpdate<__toasty_T> {
            TenantUpdate {
                assignments: ::core::clone::Clone::clone(&self.assignments),
                condition: ::core::clone::Clone::clone(&self.condition),
                target: ::core::clone::Clone::clone(&self.target),
            }
        }
    }
    impl<__toasty_T: _toasty::codegen_support::UpdateTarget> TenantUpdate<__toasty_T> {
        fn apply_update_defaults(&mut self) {}
        fn id(
            mut self,
            id: impl _toasty::codegen_support::Assign<
                <uuid::Uuid as _toasty::codegen_support::Field>::ExprTarget,
            >,
        ) -> Self {
            self.set_id(id);
            self
        }
        fn set_id(
            &mut self,
            id: impl _toasty::codegen_support::Assign<
                <uuid::Uuid as _toasty::codegen_support::Field>::ExprTarget,
            >,
        ) -> &mut Self {
            let projection = _toasty::codegen_support::stmt::Projection::from_index(0);
            id.assign(&mut self.assignments, projection);
            self
        }
        fn name(
            mut self,
            name: impl _toasty::codegen_support::Assign<
                <String as _toasty::codegen_support::Field>::ExprTarget,
            >,
        ) -> Self {
            self.set_name(name);
            self
        }
        fn set_name(
            &mut self,
            name: impl _toasty::codegen_support::Assign<
                <String as _toasty::codegen_support::Field>::ExprTarget,
            >,
        ) -> &mut Self {
            let projection = _toasty::codegen_support::stmt::Projection::from_index(1);
            name.assign(&mut self.assignments, projection);
            self
        }
        fn users(
            mut self,
            users: impl _toasty::codegen_support::Assign<
                _toasty::codegen_support::List<
                    <toasty::HasMany<User> as _toasty::codegen_support::Relation>::Expr,
                >,
            >,
        ) -> Self {
            self.set_users(users);
            self
        }
        fn set_users(
            &mut self,
            users: impl _toasty::codegen_support::Assign<
                _toasty::codegen_support::List<
                    <toasty::HasMany<User> as _toasty::codegen_support::Relation>::Expr,
                >,
            >,
        ) -> &mut Self {
            let projection = _toasty::codegen_support::stmt::Projection::from_index(2);
            users.assign(&mut self.assignments, projection);
            self
        }
        fn build_stmt(&mut self) -> _toasty::codegen_support::core::stmt::Statement {
            use _toasty::codegen_support::UpdateTarget as _;
            let assignments = ::std::mem::take(&mut self.assignments);
            let mut stmt = self.target.to_update_stmt(assignments);
            if let Some(cond) = self.condition.take() {
                stmt.as_untyped_mut().condition = _toasty::codegen_support::core::stmt::Condition::new(
                    cond,
                );
            }
            stmt.into_untyped()
        }
        async fn exec(
            mut self,
            executor: &mut dyn _toasty::codegen_support::Executor,
        ) -> _toasty::codegen_support::Result<()> {
            let response = executor.exec_untyped(self.build_stmt()).await?;
            let value = response.values.collect_as_value().await?;
            self.target.apply_result(value)?;
            Ok(())
        }
    }
    impl _toasty::codegen_support::UpdateTarget for &mut Tenant {
        type Returning = Tenant;
        fn to_update_stmt(
            &mut self,
            assignments: _toasty::codegen_support::core::stmt::Assignments,
        ) -> _toasty::codegen_support::stmt::Update<Tenant> {
            use _toasty::codegen_support::IntoStatement;
            let mut stmt = _toasty::codegen_support::stmt::Update::new(
                (&**self).into_statement().into_query().unwrap(),
            );
            stmt.set_assignments(assignments);
            stmt
        }
        fn apply_result(
            self,
            value: _toasty::codegen_support::core::stmt::Value,
        ) -> _toasty::codegen_support::Result<()> {
            <Tenant as _toasty::codegen_support::Load>::reload(self, value)
        }
    }
    impl _toasty::codegen_support::UpdateTarget for TenantQuery {
        type Returning = _toasty::codegen_support::List<Tenant>;
        fn to_update_stmt(
            &mut self,
            assignments: _toasty::codegen_support::core::stmt::Assignments,
        ) -> _toasty::codegen_support::stmt::Update<
            _toasty::codegen_support::List<Tenant>,
        > {
            let query = ::std::mem::replace(
                &mut self.stmt,
                _toasty::codegen_support::stmt::Query::all(),
            );
            let mut stmt = _toasty::codegen_support::stmt::Update::new(query);
            stmt.set_assignments(assignments);
            stmt
        }
        fn apply_result(
            self,
            _values: _toasty::codegen_support::core::stmt::Value,
        ) -> _toasty::codegen_support::Result<()> {
            Ok(())
        }
    }
    impl From<TenantQuery> for TenantUpdate {
        fn from(value: TenantQuery) -> TenantUpdate {
            let mut s = TenantUpdate {
                assignments: _toasty::codegen_support::core::stmt::Assignments::default(),
                condition: None,
                target: value,
            };
            s.apply_update_defaults();
            s
        }
    }
    impl From<
        _toasty::codegen_support::stmt::Query<_toasty::codegen_support::List<Tenant>>,
    > for TenantUpdate {
        fn from(
            src: _toasty::codegen_support::stmt::Query<
                _toasty::codegen_support::List<Tenant>,
            >,
        ) -> TenantUpdate {
            let mut s = TenantUpdate {
                assignments: _toasty::codegen_support::core::stmt::Assignments::default(),
                condition: None,
                target: TenantQuery::from_stmt(src),
            };
            s.apply_update_defaults();
            s
        }
    }
    impl _toasty::codegen_support::IntoStatement for TenantUpdate {
        type Returning = ();
        fn into_statement(mut self) -> _toasty::codegen_support::Statement<()> {
            _toasty::codegen_support::Statement::from_untyped_stmt(self.build_stmt())
        }
    }
    struct Many {
        stmt: _toasty::codegen_support::stmt::Association<
            _toasty::codegen_support::List<Tenant>,
        >,
    }
    struct One {
        stmt: _toasty::codegen_support::stmt::Query<Tenant>,
    }
    struct OptionOne {
        stmt: _toasty::codegen_support::stmt::Query<
            _toasty::codegen_support::Option<Tenant>,
        >,
    }
    impl Many {
        pub fn from_stmt(
            stmt: _toasty::codegen_support::stmt::Association<
                _toasty::codegen_support::List<Tenant>,
            >,
        ) -> Many {
            Many { stmt }
        }
        async fn get_by_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> _toasty::codegen_support::Result<Tenant> {
            self.filter_by_id(id).get(executor).await
        }
        fn update_by_id(
            self,
            id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> TenantUpdate {
            self.filter_by_id(id).update()
        }
        async fn delete_by_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> _toasty::codegen_support::Result<()> {
            self.filter_by_id(id).delete().exec(executor).await
        }
        fn filter_by_id(
            self,
            id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> TenantQuery {
            TenantQuery::from_stmt({
                    use _toasty::codegen_support::IntoStatement;
                    self.into_statement().into_query().unwrap()
                })
                .filter(Tenant::fields().id().eq(id))
        }
        fn users(
            self,
        ) -> <toasty::HasMany<User> as _toasty::codegen_support::Relation>::Many {
            <toasty::HasMany<
                User,
            > as _toasty::codegen_support::Relation>::Many::from_stmt(
                self.stmt.chain_field(2),
            )
        }
        /// Iterate all entries in the relation
        async fn exec(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
        ) -> _toasty::codegen_support::Result<Vec<Tenant>> {
            use _toasty::codegen_support::IntoStatement;
            self.into_statement().exec(executor).await
        }
        fn filter(
            self,
            filter: _toasty::codegen_support::stmt::Expr<bool>,
        ) -> TenantQuery {
            use _toasty::codegen_support::IntoStatement;
            let select = self.into_statement().into_query().unwrap();
            TenantQuery::from_stmt(select.and(filter))
        }
        fn create(self) -> TenantCreate {
            let mut builder = TenantCreate::default();
            builder.stmt.set_scope(self.stmt);
            builder
        }
        /// Add an item to the association
        async fn insert(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            item: impl _toasty::codegen_support::IntoExpr<Tenant>,
        ) -> _toasty::codegen_support::Result<()> {
            executor.exec(self.stmt.insert(item)).await
        }
        /// Remove items from the association
        async fn remove(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            item: impl _toasty::codegen_support::IntoExpr<Tenant>,
        ) -> _toasty::codegen_support::Result<()> {
            executor.exec(self.stmt.remove(item)).await
        }
    }
    impl _toasty::codegen_support::IntoStatement for Many {
        type Returning = _toasty::codegen_support::List<Tenant>;
        fn into_statement(
            self,
        ) -> _toasty::codegen_support::Statement<
            _toasty::codegen_support::List<Tenant>,
        > {
            use _toasty::codegen_support::IntoStatement;
            self.stmt.into_statement()
        }
    }
    impl One {
        fn from_stmt(
            stmt: _toasty::codegen_support::stmt::Query<
                _toasty::codegen_support::List<Tenant>,
            >,
        ) -> One {
            One { stmt: stmt.one() }
        }
        /// Create a new associated record
        fn create(self) -> TenantCreate {
            let mut builder = TenantCreate::default();
            builder.stmt.set_scope(self.stmt);
            builder
        }
        async fn exec(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
        ) -> _toasty::codegen_support::Result<Tenant> {
            self.stmt.exec(executor).await
        }
    }
    impl _toasty::codegen_support::IntoStatement for One {
        type Returning = Tenant;
        fn into_statement(self) -> _toasty::codegen_support::Statement<Tenant> {
            use _toasty::codegen_support::IntoStatement;
            self.stmt.into_statement()
        }
    }
    impl OptionOne {
        pub fn from_stmt(
            stmt: _toasty::codegen_support::stmt::Query<
                _toasty::codegen_support::List<Tenant>,
            >,
        ) -> OptionOne {
            OptionOne { stmt: stmt.first() }
        }
        /// Create a new associated record
        fn create(self) -> TenantCreate {
            let mut builder = TenantCreate::default();
            builder.stmt.set_scope(self.stmt);
            builder
        }
        async fn exec(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
        ) -> _toasty::codegen_support::Result<_toasty::codegen_support::Option<Tenant>> {
            self.stmt.exec(executor).await
        }
    }
    #[diagnostic::do_not_recommend]
    impl _toasty::codegen_support::Scope for Many {
        type Item = _toasty::codegen_support::List<Tenant>;
        type Path<__Origin> = TenantListFields<__Origin>;
        type Create = TenantCreate;
        fn new_path<__Origin>(
            path: _toasty::codegen_support::Path<__Origin, Self::Item>,
        ) -> Self::Path<__Origin> {
            TenantListFields::from_path(path)
        }
        fn new_create() -> Self::Create {
            TenantCreate::default()
        }
        fn new_path_root() -> Self::Path<Self::Item> {
            TenantListFields::from_path(
                _toasty::codegen_support::Path::from_model_list(),
            )
        }
    }
    #[diagnostic::do_not_recommend]
    impl _toasty::codegen_support::Scope for One {
        type Item = Tenant;
        type Path<__Origin> = TenantFields<__Origin>;
        type Create = TenantCreate;
        fn new_path<__Origin>(
            path: _toasty::codegen_support::Path<__Origin, Self::Item>,
        ) -> Self::Path<__Origin> {
            TenantFields::from_path(path)
        }
        fn new_create() -> Self::Create {
            TenantCreate::default()
        }
        fn new_path_root() -> Self::Path<Self::Item> {
            TenantFields::from_path(_toasty::codegen_support::Path::root())
        }
    }
    #[diagnostic::do_not_recommend]
    impl _toasty::codegen_support::Scope for OptionOne {
        type Item = Tenant;
        type Path<__Origin> = TenantFields<__Origin>;
        type Create = TenantCreate;
        fn new_path<__Origin>(
            path: _toasty::codegen_support::Path<__Origin, Self::Item>,
        ) -> Self::Path<__Origin> {
            TenantFields::from_path(path)
        }
        fn new_create() -> Self::Create {
            TenantCreate::default()
        }
        fn new_path_root() -> Self::Path<Self::Item> {
            TenantFields::from_path(_toasty::codegen_support::Path::root())
        }
    }
    #[diagnostic::do_not_recommend]
    impl _toasty::codegen_support::ValidateCreate for Many {
        const CREATE_META: &'static _toasty::codegen_support::CreateMeta = &<Tenant as _toasty::codegen_support::Model>::CREATE_META;
    }
    #[diagnostic::do_not_recommend]
    impl _toasty::codegen_support::ValidateCreate for One {
        const CREATE_META: &'static _toasty::codegen_support::CreateMeta = &<Tenant as _toasty::codegen_support::Model>::CREATE_META;
    }
    #[diagnostic::do_not_recommend]
    impl _toasty::codegen_support::ValidateCreate for OptionOne {
        const CREATE_META: &'static _toasty::codegen_support::CreateMeta = &<Tenant as _toasty::codegen_support::Model>::CREATE_META;
    }
    #[diagnostic::do_not_recommend]
    impl<__Origin> _toasty::codegen_support::ValidateCreate for TenantFields<__Origin> {
        const CREATE_META: &'static _toasty::codegen_support::CreateMeta = &<Tenant as _toasty::codegen_support::Model>::CREATE_META;
    }
    #[diagnostic::do_not_recommend]
    impl<__Origin> _toasty::codegen_support::ValidateCreate
    for TenantListFields<__Origin> {
        const CREATE_META: &'static _toasty::codegen_support::CreateMeta = &<Tenant as _toasty::codegen_support::Model>::CREATE_META;
    }
};
#[item_collection(Tenant)]
#[key(partition = tenant_id, local = id)]
struct User {
    id: String,
    tenant_id: uuid::Uuid,
    #[belongs_to(key = tenant_id, references = id)]
    tenant: toasty::BelongsTo<Tenant>,
    name: String,
    #[has_many]
    todos: toasty::HasMany<Todo>,
}
#[automatically_derived]
impl ::core::fmt::Debug for User {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field5_finish(
            f,
            "User",
            "id",
            &self.id,
            "tenant_id",
            &self.tenant_id,
            "tenant",
            &self.tenant,
            "name",
            &self.name,
            "todos",
            &&self.todos,
        )
    }
}
const _: () = {
    use toasty as _toasty;
    impl User {
        fn fields() -> UserFields<User> {
            UserFields {
                path: _toasty::codegen_support::Path::root(),
            }
        }
        async fn get_by_tenant_id_and_id(
            executor: &mut dyn _toasty::codegen_support::Executor,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> _toasty::codegen_support::Result<User> {
            Self::filter_by_tenant_id_and_id(tenant_id, id).get(executor).await
        }
        fn update_by_tenant_id_and_id(
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> UserUpdate {
            Self::filter_by_tenant_id_and_id(tenant_id, id).update()
        }
        async fn delete_by_tenant_id_and_id(
            executor: &mut dyn _toasty::codegen_support::Executor,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> _toasty::codegen_support::Result<()> {
            Self::filter_by_tenant_id_and_id(tenant_id, id).delete().exec(executor).await
        }
        fn filter_by_tenant_id_and_id(
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> UserQuery {
            UserQuery::default().filter_by_tenant_id_and_id(tenant_id, id)
        }
        async fn get_by_tenant_id(
            executor: &mut dyn _toasty::codegen_support::Executor,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> _toasty::codegen_support::Result<User> {
            Self::filter_by_tenant_id(tenant_id).get(executor).await
        }
        fn update_by_tenant_id(
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> UserUpdate {
            Self::filter_by_tenant_id(tenant_id).update()
        }
        async fn delete_by_tenant_id(
            executor: &mut dyn _toasty::codegen_support::Executor,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> _toasty::codegen_support::Result<()> {
            Self::filter_by_tenant_id(tenant_id).delete().exec(executor).await
        }
        fn filter_by_tenant_id(
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> UserQuery {
            UserQuery::default().filter_by_tenant_id(tenant_id)
        }
        fn tenant(
            &self,
        ) -> <toasty::BelongsTo<Tenant> as _toasty::codegen_support::Relation>::One {
            if false {
                let _ = &self.tenant;
            }
            {
                use _toasty::codegen_support::IntoStatement;
                <toasty::BelongsTo<
                    Tenant,
                > as _toasty::codegen_support::Relation>::One::from_stmt(
                    <toasty::BelongsTo<
                        Tenant,
                    > as _toasty::codegen_support::Relation>::Model::filter(
                            _toasty::codegen_support::Field::key_constraint(
                                &self.tenant_id,
                                <toasty::BelongsTo<
                                    Tenant,
                                > as _toasty::codegen_support::Relation>::Model::fields()
                                    .id()
                                    .into(),
                            ),
                        )
                        .into_statement()
                        .into_query()
                        .unwrap(),
                )
            }
        }
        #[doc(hidden)]
        fn verify_pair_belongs_to_exists_for_tenant(
            &self,
        ) -> &toasty::BelongsTo<Tenant> {
            let _ = &self.tenant_id;
            &self.tenant
        }
        fn todos(
            &self,
        ) -> <toasty::HasMany<Todo> as _toasty::codegen_support::Relation>::Many {
            if false {
                let _ = &self.todos;
            }
            #[allow(unreachable_code)]
            if false {
                fn load<__toasty_T: _toasty::codegen_support::Model>() -> __toasty_T {
                    __toasty_T::load(::core::panicking::panic("not yet implemented"))
                        .unwrap()
                }
                #[diagnostic::on_unimplemented(
                    message = "HasMany requires the {__toasty_A}::user field to be of type `BelongsTo<Self>`, but it was `{Self}` instead",
                    label = "Has many associations require the target to include a back-reference",
                    note = "Note 1",
                )]
                trait Verify<__toasty_A> {}
                #[diagnostic::do_not_recommend]
                impl<__toasty_A> Verify<__toasty_A>
                for _toasty::codegen_support::BelongsTo<User> {}
                #[diagnostic::do_not_recommend]
                impl<__toasty_A> Verify<__toasty_A>
                for _toasty::codegen_support::BelongsTo<Option<User>> {}
                fn verify<__toasty_T: Verify<__toasty_A>, __toasty_A>(_: &__toasty_T) {}
                let instance = load::<
                    <toasty::HasMany<Todo> as _toasty::codegen_support::Relation>::Model,
                >();
                verify::<
                    _,
                    <toasty::HasMany<Todo> as _toasty::codegen_support::Relation>::Model,
                >(instance.verify_pair_belongs_to_exists_for_user());
            }
            {
                use _toasty::codegen_support::IntoStatement;
                <toasty::HasMany<
                    Todo,
                > as _toasty::codegen_support::Relation>::Many::from_stmt(
                    _toasty::codegen_support::stmt::Association::many(
                        self.into_statement().into_query().unwrap().to_list(),
                        Self::fields().todos().into(),
                    ),
                )
            }
        }
        #[doc(hidden)]
        pub const fn __check_create_fields(__provided: &[&str]) {
            if !<String as _toasty::codegen_support::Field>::NULLABLE
                && !_toasty::codegen_support::const_contains(__provided, "id")
            {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("missing required field `id` in create! for `User`"),
                    );
                };
            }
            if !<String as _toasty::codegen_support::Field>::NULLABLE
                && !_toasty::codegen_support::const_contains(__provided, "name")
            {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "missing required field `name` in create! for `User`",
                        ),
                    );
                };
            }
        }
        fn create() -> UserCreate {
            UserCreate::default()
        }
        fn create_many() -> _toasty::codegen_support::stmt::CreateMany<User> {
            _toasty::codegen_support::stmt::CreateMany::default()
        }
        fn update(&mut self) -> UserUpdate<&mut Self> {
            let mut s = UserUpdate {
                assignments: _toasty::codegen_support::core::stmt::Assignments::default(),
                condition: None,
                target: self,
            };
            s.apply_update_defaults();
            s
        }
        fn all() -> UserQuery {
            UserQuery::default()
        }
        fn filter(expr: _toasty::codegen_support::stmt::Expr<bool>) -> UserQuery {
            UserQuery::from_stmt(_toasty::codegen_support::stmt::Query::filter(expr))
        }
        fn delete(self) -> _toasty::codegen_support::stmt::Delete<()> {
            {
                let __delete = UserQuery::default()
                    .filter_by_tenant_id_and_id(&self.tenant_id, &self.id)
                    .delete();
                __delete
            }
        }
    }
    impl _toasty::codegen_support::Register for User {
        fn id() -> _toasty::codegen_support::core::schema::app::ModelId {
            static ID: std::sync::OnceLock<
                _toasty::codegen_support::core::schema::app::ModelId,
            > = std::sync::OnceLock::new();
            *ID.get_or_init(|| _toasty::codegen_support::generate_unique_id())
        }
        fn schema() -> _toasty::codegen_support::core::schema::app::Model {
            let id = User::id();
            _toasty::codegen_support::core::schema::app::Model::Root(_toasty::codegen_support::core::schema::app::ModelRoot {
                id,
                name: _toasty::codegen_support::core::schema::Name {
                    parts: ::alloc::boxed::box_assume_init_into_vec_unsafe(
                        ::alloc::intrinsics::write_box_via_move(
                            ::alloc::boxed::Box::new_uninit(),
                            ["user".to_string()],
                        ),
                    ),
                },
                fields: ::alloc::boxed::box_assume_init_into_vec_unsafe(
                    ::alloc::intrinsics::write_box_via_move(
                        ::alloc::boxed::Box::new_uninit(),
                        [
                            _toasty::codegen_support::core::schema::app::Field {
                                id: _toasty::codegen_support::core::schema::app::FieldId {
                                    model: User::id(),
                                    index: 0,
                                },
                                name: _toasty::codegen_support::core::schema::app::FieldName {
                                    app: Some("id".to_string()),
                                    storage: None,
                                },
                                ty: <String as _toasty::codegen_support::Field>::field_ty(
                                    None,
                                ),
                                nullable: <String as _toasty::codegen_support::Field>::NULLABLE,
                                primary_key: true,
                                auto: None,
                                versionable: false,
                                deferred: false,
                                constraints: ::alloc::vec::Vec::new(),
                                variant: None,
                            },
                            _toasty::codegen_support::core::schema::app::Field {
                                id: _toasty::codegen_support::core::schema::app::FieldId {
                                    model: User::id(),
                                    index: 1,
                                },
                                name: _toasty::codegen_support::core::schema::app::FieldName {
                                    app: Some("tenant_id".to_string()),
                                    storage: None,
                                },
                                ty: <uuid::Uuid as _toasty::codegen_support::Field>::field_ty(
                                    None,
                                ),
                                nullable: <uuid::Uuid as _toasty::codegen_support::Field>::NULLABLE,
                                primary_key: true,
                                auto: None,
                                versionable: false,
                                deferred: false,
                                constraints: ::alloc::vec::Vec::new(),
                                variant: None,
                            },
                            _toasty::codegen_support::core::schema::app::Field {
                                id: _toasty::codegen_support::core::schema::app::FieldId {
                                    model: User::id(),
                                    index: 2,
                                },
                                name: _toasty::codegen_support::core::schema::app::FieldName {
                                    app: Some("tenant".to_string()),
                                    storage: None,
                                },
                                ty: <toasty::BelongsTo<
                                    Tenant,
                                > as _toasty::codegen_support::Relation>::belongs_to_field_ty(_toasty::codegen_support::core::schema::app::ForeignKey {
                                    fields: ::alloc::boxed::box_assume_init_into_vec_unsafe(
                                        ::alloc::intrinsics::write_box_via_move(
                                            ::alloc::boxed::Box::new_uninit(),
                                            [
                                                _toasty::codegen_support::core::schema::app::ForeignKeyField {
                                                    source: _toasty::codegen_support::core::schema::app::FieldId {
                                                        model: User::id(),
                                                        index: 1,
                                                    },
                                                    target: <toasty::BelongsTo<
                                                        Tenant,
                                                    > as _toasty::codegen_support::Relation>::field_name_to_id(
                                                        "id",
                                                    ),
                                                },
                                            ],
                                        ),
                                    ),
                                }),
                                nullable: <toasty::BelongsTo<
                                    Tenant,
                                > as _toasty::codegen_support::Relation>::nullable(),
                                primary_key: false,
                                auto: None,
                                versionable: false,
                                deferred: false,
                                constraints: ::alloc::vec::Vec::new(),
                                variant: None,
                            },
                            _toasty::codegen_support::core::schema::app::Field {
                                id: _toasty::codegen_support::core::schema::app::FieldId {
                                    model: User::id(),
                                    index: 3,
                                },
                                name: _toasty::codegen_support::core::schema::app::FieldName {
                                    app: Some("name".to_string()),
                                    storage: None,
                                },
                                ty: <String as _toasty::codegen_support::Field>::field_ty(
                                    None,
                                ),
                                nullable: <String as _toasty::codegen_support::Field>::NULLABLE,
                                primary_key: false,
                                auto: None,
                                versionable: false,
                                deferred: false,
                                constraints: ::alloc::vec::Vec::new(),
                                variant: None,
                            },
                            _toasty::codegen_support::core::schema::app::Field {
                                id: _toasty::codegen_support::core::schema::app::FieldId {
                                    model: User::id(),
                                    index: 4,
                                },
                                name: _toasty::codegen_support::core::schema::app::FieldName {
                                    app: Some("todos".to_string()),
                                    storage: None,
                                },
                                ty: <toasty::HasMany<
                                    Todo,
                                > as _toasty::codegen_support::Relation>::has_many_field_ty(
                                    _toasty::codegen_support::core::schema::Name {
                                        parts: ::alloc::boxed::box_assume_init_into_vec_unsafe(
                                            ::alloc::intrinsics::write_box_via_move(
                                                ::alloc::boxed::Box::new_uninit(),
                                                ["todo".to_string()],
                                            ),
                                        ),
                                    },
                                    None,
                                    None,
                                ),
                                nullable: <toasty::HasMany<
                                    Todo,
                                > as _toasty::codegen_support::Relation>::nullable(),
                                primary_key: false,
                                auto: None,
                                versionable: false,
                                deferred: false,
                                constraints: ::alloc::vec::Vec::new(),
                                variant: None,
                            },
                        ],
                    ),
                ),
                primary_key: _toasty::codegen_support::core::schema::app::PrimaryKey {
                    fields: ::alloc::boxed::box_assume_init_into_vec_unsafe(
                        ::alloc::intrinsics::write_box_via_move(
                            ::alloc::boxed::Box::new_uninit(),
                            [
                                _toasty::codegen_support::core::schema::app::FieldId {
                                    model: id,
                                    index: 1,
                                },
                                _toasty::codegen_support::core::schema::app::FieldId {
                                    model: id,
                                    index: 0,
                                },
                            ],
                        ),
                    ),
                    index: _toasty::codegen_support::core::schema::app::IndexId {
                        model: id,
                        index: 0,
                    },
                },
                table_name: None,
                item_collection: Some(
                    <Tenant as _toasty::codegen_support::Register>::id(),
                ),
                indices: ::alloc::boxed::box_assume_init_into_vec_unsafe(
                    ::alloc::intrinsics::write_box_via_move(
                        ::alloc::boxed::Box::new_uninit(),
                        [
                            _toasty::codegen_support::core::schema::app::Index {
                                id: _toasty::codegen_support::core::schema::app::IndexId {
                                    model: id,
                                    index: 0,
                                },
                                name: None,
                                fields: ::alloc::boxed::box_assume_init_into_vec_unsafe(
                                    ::alloc::intrinsics::write_box_via_move(
                                        ::alloc::boxed::Box::new_uninit(),
                                        [
                                            _toasty::codegen_support::core::schema::app::IndexField {
                                                field: _toasty::codegen_support::core::schema::app::FieldId {
                                                    model: id,
                                                    index: 1,
                                                },
                                                op: _toasty::codegen_support::core::schema::db::IndexOp::Eq,
                                                scope: _toasty::codegen_support::core::schema::db::IndexScope::Partition,
                                            },
                                            _toasty::codegen_support::core::schema::app::IndexField {
                                                field: _toasty::codegen_support::core::schema::app::FieldId {
                                                    model: id,
                                                    index: 0,
                                                },
                                                op: _toasty::codegen_support::core::schema::db::IndexOp::Eq,
                                                scope: _toasty::codegen_support::core::schema::db::IndexScope::Local,
                                            },
                                        ],
                                    ),
                                ),
                                unique: true,
                                primary_key: true,
                            },
                        ],
                    ),
                ),
                version_field: None,
            })
        }
        fn register(
            model_set: &mut _toasty::codegen_support::core::schema::app::ModelSet,
        ) {
            if model_set.contains(Self::id()) {
                return;
            }
            model_set.add(Self::schema());
            <String as _toasty::codegen_support::Field>::register(model_set);
            <uuid::Uuid as _toasty::codegen_support::Field>::register(model_set);
            <<toasty::BelongsTo<
                Tenant,
            > as _toasty::codegen_support::Relation>::Model as _toasty::codegen_support::Register>::register(
                model_set,
            );
            <String as _toasty::codegen_support::Field>::register(model_set);
            <<toasty::HasMany<
                Todo,
            > as _toasty::codegen_support::Relation>::Model as _toasty::codegen_support::Register>::register(
                model_set,
            );
        }
    }
    #[allow(non_upper_case_globals)]
    const _: () = {
        static __INVENTORY: ::inventory::Node = ::inventory::Node {
            value: &{
                _toasty::codegen_support::DiscoverItem::new(
                    "example-item-collection",
                    |model_set| {
                        <User as _toasty::codegen_support::Register>::register(
                            model_set,
                        );
                    },
                )
            },
            next: ::inventory::__private::UnsafeCell::new(
                ::inventory::__private::Option::None,
            ),
        };
        #[link_section = ".text.startup"]
        unsafe extern "C" fn __ctor() {
            unsafe { ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY) }
        }
        #[used]
        #[link_section = ".init_array"]
        static __CTOR: unsafe extern "C" fn() = __ctor;
    };
    impl _toasty::codegen_support::Load for User {
        type Output = Self;
        fn ty() -> _toasty::codegen_support::core::stmt::Type {
            _toasty::codegen_support::core::stmt::Type::Model(
                <Self as _toasty::codegen_support::Register>::id(),
            )
        }
        fn load(
            value: _toasty::codegen_support::core::stmt::Value,
        ) -> _toasty::codegen_support::Result<Self> {
            match value {
                _toasty::codegen_support::core::stmt::Value::Null => {
                    Err(_toasty::codegen_support::Error::record_not_found("User"))
                }
                _toasty::codegen_support::core::stmt::Value::Record(mut record) => {
                    Ok(User {
                        id: <String as _toasty::codegen_support::Load>::load(
                            record[0].take(),
                        )?,
                        tenant_id: <uuid::Uuid as _toasty::codegen_support::Load>::load(
                            record[1].take(),
                        )?,
                        tenant: _toasty::codegen_support::BelongsTo::load(
                            record[2usize].take(),
                        )?,
                        name: <String as _toasty::codegen_support::Load>::load(
                            record[3].take(),
                        )?,
                        todos: _toasty::codegen_support::HasMany::load(
                            record[4usize].take(),
                        )?,
                    })
                }
                value => {
                    Err(_toasty::codegen_support::Error::type_conversion(value, "User"))
                }
            }
        }
        fn reload(
            target: &mut Self,
            value: _toasty::codegen_support::core::stmt::Value,
        ) -> _toasty::codegen_support::Result<()> {
            for (field, value) in value.into_sparse_record().into_iter() {
                match field {
                    0 => {
                        <String as _toasty::codegen_support::Load>::reload(
                            &mut target.id,
                            value,
                        )?
                    }
                    1 => {
                        <uuid::Uuid as _toasty::codegen_support::Load>::reload(
                            &mut target.tenant_id,
                            value,
                        )?
                    }
                    2 => target.tenant.unload(),
                    3 => {
                        <String as _toasty::codegen_support::Load>::reload(
                            &mut target.name,
                            value,
                        )?
                    }
                    4 => target.todos.unload(),
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "not yet implemented: {0}",
                                format_args!(
                                    "handle unknown field id in reload after update",
                                ),
                            ),
                        );
                    }
                }
            }
            Ok(())
        }
    }
    impl _toasty::codegen_support::Model for User {
        type Query = UserQuery;
        type Create = UserCreate;
        type Update<'a> = UserUpdate<&'a mut Self>;
        type UpdateQuery = UserUpdate;
        type Path<__Origin> = UserFields<__Origin>;
        type PrimaryKey = (uuid::Uuid, String);
        const CREATE_META: _toasty::codegen_support::CreateMeta = _toasty::codegen_support::CreateMeta {
            fields: &[
                _toasty::codegen_support::CreateField {
                    name: "id",
                    required: !<String as _toasty::codegen_support::Field>::NULLABLE,
                },
                _toasty::codegen_support::CreateField {
                    name: "name",
                    required: !<String as _toasty::codegen_support::Field>::NULLABLE,
                },
            ],
            model_name: "User",
        };
        fn new_path<__Origin>(
            path: _toasty::codegen_support::Path<__Origin, Self>,
        ) -> Self::Path<__Origin> {
            UserFields::from_path(path)
        }
        fn find_by_primary_key(
            id: _toasty::codegen_support::stmt::Expr<Self::PrimaryKey>,
        ) -> Self::Query {
            let pk_expr: _toasty::codegen_support::stmt::Expr<Self::PrimaryKey> = _toasty::codegen_support::IntoExpr::into_expr((
                User::fields().tenant_id(),
                User::fields().id(),
            ));
            Self::filter(pk_expr.eq(id))
        }
    }
    impl _toasty::codegen_support::Relation for User {
        type Model = User;
        type Expr = User;
        type Query = UserQuery;
        type Create = UserCreate;
        type Many = Many;
        type ManyField<__Origin> = UserListFields<__Origin>;
        type One = One;
        type OneField<__Origin> = UserFields<__Origin>;
        type OptionOne = OptionOne;
        fn new_many_field<__Origin>(
            path: _toasty::codegen_support::Path<
                __Origin,
                _toasty::codegen_support::List<Self::Model>,
            >,
        ) -> UserListFields<__Origin> {
            UserListFields::from_path(path)
        }
        fn field_name_to_id(
            name: &str,
        ) -> _toasty::codegen_support::core::schema::app::FieldId {
            use _toasty::codegen_support::{Model, Register};
            match name {
                "id" => {
                    _toasty::codegen_support::core::schema::app::FieldId {
                        model: Self::id(),
                        index: 0,
                    }
                }
                "tenant_id" => {
                    _toasty::codegen_support::core::schema::app::FieldId {
                        model: Self::id(),
                        index: 1,
                    }
                }
                "tenant" => {
                    _toasty::codegen_support::core::schema::app::FieldId {
                        model: Self::id(),
                        index: 2,
                    }
                }
                "name" => {
                    _toasty::codegen_support::core::schema::app::FieldId {
                        model: Self::id(),
                        index: 3,
                    }
                }
                "todos" => {
                    _toasty::codegen_support::core::schema::app::FieldId {
                        model: Self::id(),
                        index: 4,
                    }
                }
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "not yet implemented: {0}",
                            format_args!("field_name_to_id: {0}", name),
                        ),
                    );
                }
            }
        }
    }
    impl _toasty::codegen_support::stmt::IntoExpr<User> for User {
        fn into_expr(self) -> _toasty::codegen_support::stmt::Expr<User> {
            let expr: _toasty::codegen_support::stmt::Expr<(uuid::Uuid, String)> = _toasty::codegen_support::IntoExpr::into_expr((
                self.tenant_id,
                self.id,
            ));
            expr.cast()
        }
        fn by_ref(&self) -> _toasty::codegen_support::stmt::Expr<User> {
            let expr: _toasty::codegen_support::stmt::Expr<(uuid::Uuid, String)> = _toasty::codegen_support::IntoExpr::into_expr((
                &self.tenant_id,
                &self.id,
            ));
            expr.cast()
        }
    }
    impl _toasty::codegen_support::Assign<User> for User {
        fn into_assignment(self) -> _toasty::codegen_support::stmt::Assignment<User> {
            _toasty::codegen_support::stmt::set(
                <Self as _toasty::codegen_support::IntoExpr<User>>::into_expr(self),
            )
        }
    }
    impl _toasty::codegen_support::IntoStatement for &User {
        type Returning = User;
        fn into_statement(self) -> _toasty::codegen_support::Statement<User> {
            use _toasty::codegen_support::IntoStatement;
            _toasty::codegen_support::IntoStatement::into_statement(
                UserQuery::default()
                    .filter_by_tenant_id_and_id(&self.tenant_id, &self.id)
                    .one(),
            )
        }
    }
    impl _toasty::codegen_support::IntoStatement for &mut User {
        type Returning = User;
        fn into_statement(self) -> _toasty::codegen_support::Statement<User> {
            (&*self).into_statement()
        }
    }
    impl _toasty::codegen_support::IntoStatement for User {
        type Returning = User;
        fn into_statement(self) -> _toasty::codegen_support::Statement<User> {
            (&self).into_statement()
        }
    }
    struct UserFields<__Origin> {
        path: _toasty::codegen_support::Path<__Origin, User>,
    }
    impl<__Origin> UserFields<__Origin> {
        const fn from_path(
            path: _toasty::codegen_support::Path<__Origin, User>,
        ) -> UserFields<__Origin> {
            UserFields { path }
        }
        fn path(&self) -> _toasty::codegen_support::Path<__Origin, User> {
            self.path.clone()
        }
        fn eq(
            self,
            rhs: impl _toasty::codegen_support::IntoExpr<User>,
        ) -> _toasty::codegen_support::stmt::Expr<bool> {
            use _toasty::codegen_support::IntoExpr;
            self.path.eq(rhs.into_expr())
        }
        fn in_query(
            self,
            rhs: impl _toasty::codegen_support::IntoStatement<
                Returning = _toasty::codegen_support::List<User>,
            >,
        ) -> _toasty::codegen_support::stmt::Expr<bool> {
            self.path.in_query(rhs)
        }
        fn create(&self) -> UserCreate {
            UserCreate::default()
        }
        fn id(&self) -> <String as _toasty::codegen_support::Field>::Path<__Origin> {
            <String as _toasty::codegen_support::Field>::new_path(
                self
                    .path()
                    .chain(
                        _toasty::codegen_support::Path::<
                            User,
                            <String as _toasty::codegen_support::Field>::ExprTarget,
                        >::from_field_index(0),
                    ),
            )
        }
        fn tenant_id(
            &self,
        ) -> <uuid::Uuid as _toasty::codegen_support::Field>::Path<__Origin> {
            <uuid::Uuid as _toasty::codegen_support::Field>::new_path(
                self
                    .path()
                    .chain(
                        _toasty::codegen_support::Path::<
                            User,
                            <uuid::Uuid as _toasty::codegen_support::Field>::ExprTarget,
                        >::from_field_index(1),
                    ),
            )
        }
        fn tenant(
            &self,
        ) -> <toasty::BelongsTo<
            Tenant,
        > as _toasty::codegen_support::Relation>::OneField<__Origin> {
            <toasty::BelongsTo<
                Tenant,
            > as _toasty::codegen_support::Relation>::OneField::from_path(
                self
                    .path()
                    .chain(
                        _toasty::codegen_support::Path::<User, _>::from_field_index(2),
                    ),
            )
        }
        fn name(&self) -> <String as _toasty::codegen_support::Field>::Path<__Origin> {
            <String as _toasty::codegen_support::Field>::new_path(
                self
                    .path()
                    .chain(
                        _toasty::codegen_support::Path::<
                            User,
                            <String as _toasty::codegen_support::Field>::ExprTarget,
                        >::from_field_index(3),
                    ),
            )
        }
        fn todos(
            &self,
        ) -> <toasty::HasMany<
            Todo,
        > as _toasty::codegen_support::Relation>::ManyField<__Origin> {
            <toasty::HasMany<
                Todo,
            > as _toasty::codegen_support::Relation>::ManyField::from_path(
                self
                    .path()
                    .chain(
                        _toasty::codegen_support::Path::<User, _>::from_field_index(4),
                    ),
            )
        }
    }
    impl<__Origin> Into<_toasty::codegen_support::Path<__Origin, User>>
    for UserFields<__Origin> {
        fn into(self) -> _toasty::codegen_support::Path<__Origin, User> {
            self.path
        }
    }
    impl<__Origin> _toasty::codegen_support::IntoExpr<User> for UserFields<__Origin> {
        fn into_expr(self) -> _toasty::codegen_support::stmt::Expr<User> {
            self.path.into_expr()
        }
        fn by_ref(&self) -> _toasty::codegen_support::stmt::Expr<User> {
            self.path.by_ref()
        }
    }
    struct UserListFields<__Origin> {
        path: _toasty::codegen_support::Path<
            __Origin,
            _toasty::codegen_support::List<User>,
        >,
    }
    impl<__Origin> UserListFields<__Origin> {
        const fn from_path(
            path: _toasty::codegen_support::Path<
                __Origin,
                _toasty::codegen_support::List<User>,
            >,
        ) -> UserListFields<__Origin> {
            UserListFields { path }
        }
        fn path(
            &self,
        ) -> _toasty::codegen_support::Path<
            __Origin,
            _toasty::codegen_support::List<User>,
        > {
            self.path.clone()
        }
        /// Filter the parent model by a condition on the associated
        /// (child) model. Returns `true` when **any** associated record
        /// satisfies `filter`.
        fn any(
            self,
            filter: _toasty::codegen_support::stmt::Expr<bool>,
        ) -> _toasty::codegen_support::stmt::Expr<bool> {
            self.path.any(filter)
        }
        /// Filter the parent model by a condition on the associated
        /// (child) model. Returns `true` when **all** associated records
        /// satisfy `filter` (vacuously true when there are no
        /// associated records).
        fn all(
            self,
            filter: _toasty::codegen_support::stmt::Expr<bool>,
        ) -> _toasty::codegen_support::stmt::Expr<bool> {
            self.path.all(filter)
        }
        fn create(&self) -> UserCreate {
            UserCreate::default()
        }
        fn id(&self) -> <String as _toasty::codegen_support::Field>::ListPath<__Origin> {
            <String as _toasty::codegen_support::Field>::new_list_path(
                self
                    .path()
                    .chain(
                        _toasty::codegen_support::Path::<User, _>::from_field_index(0),
                    ),
            )
        }
        fn tenant_id(
            &self,
        ) -> <uuid::Uuid as _toasty::codegen_support::Field>::ListPath<__Origin> {
            <uuid::Uuid as _toasty::codegen_support::Field>::new_list_path(
                self
                    .path()
                    .chain(
                        _toasty::codegen_support::Path::<User, _>::from_field_index(1),
                    ),
            )
        }
        fn tenant(
            &self,
        ) -> <toasty::BelongsTo<
            Tenant,
        > as _toasty::codegen_support::Relation>::ManyField<__Origin> {
            <toasty::BelongsTo<
                Tenant,
            > as _toasty::codegen_support::Relation>::ManyField::from_path(
                self
                    .path()
                    .chain(
                        _toasty::codegen_support::Path::<User, _>::from_field_index(2),
                    ),
            )
        }
        fn name(
            &self,
        ) -> <String as _toasty::codegen_support::Field>::ListPath<__Origin> {
            <String as _toasty::codegen_support::Field>::new_list_path(
                self
                    .path()
                    .chain(
                        _toasty::codegen_support::Path::<User, _>::from_field_index(3),
                    ),
            )
        }
        fn todos(
            &self,
        ) -> <toasty::HasMany<
            Todo,
        > as _toasty::codegen_support::Relation>::ManyField<__Origin> {
            <toasty::HasMany<
                Todo,
            > as _toasty::codegen_support::Relation>::ManyField::from_path(
                self
                    .path()
                    .chain(
                        _toasty::codegen_support::Path::<User, _>::from_field_index(4),
                    ),
            )
        }
    }
    impl<
        __Origin,
    > Into<
        _toasty::codegen_support::Path<__Origin, _toasty::codegen_support::List<User>>,
    > for UserListFields<__Origin> {
        fn into(
            self,
        ) -> _toasty::codegen_support::Path<
            __Origin,
            _toasty::codegen_support::List<User>,
        > {
            self.path
        }
    }
    impl<
        __Origin,
    > _toasty::codegen_support::IntoExpr<_toasty::codegen_support::List<User>>
    for UserListFields<__Origin> {
        fn into_expr(
            self,
        ) -> _toasty::codegen_support::stmt::Expr<_toasty::codegen_support::List<User>> {
            self.path.into_expr()
        }
        fn by_ref(
            &self,
        ) -> _toasty::codegen_support::stmt::Expr<_toasty::codegen_support::List<User>> {
            self.path.by_ref()
        }
    }
    struct UserQuery {
        stmt: _toasty::codegen_support::stmt::Query<
            _toasty::codegen_support::List<User>,
        >,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for UserQuery {
        #[inline]
        fn clone(&self) -> UserQuery {
            UserQuery {
                stmt: ::core::clone::Clone::clone(&self.stmt),
            }
        }
    }
    impl UserQuery {
        const fn from_stmt(
            stmt: _toasty::codegen_support::stmt::Query<
                _toasty::codegen_support::List<User>,
            >,
        ) -> UserQuery {
            UserQuery { stmt }
        }
        async fn get_by_tenant_id_and_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> _toasty::codegen_support::Result<User> {
            self.filter_by_tenant_id_and_id(tenant_id, id).get(executor).await
        }
        fn update_by_tenant_id_and_id(
            self,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> UserUpdate {
            self.filter_by_tenant_id_and_id(tenant_id, id).update()
        }
        async fn delete_by_tenant_id_and_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> _toasty::codegen_support::Result<()> {
            self.filter_by_tenant_id_and_id(tenant_id, id).delete().exec(executor).await
        }
        fn filter_by_tenant_id_and_id(
            self,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> UserQuery {
            self.filter(
                _toasty::codegen_support::stmt::Expr::and_all([
                    User::fields().tenant_id().eq(tenant_id),
                    User::fields().id().eq(id),
                ]),
            )
        }
        async fn get_by_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> _toasty::codegen_support::Result<User> {
            self.filter_by_id(id).get(executor).await
        }
        fn update_by_id(
            self,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> UserUpdate {
            self.filter_by_id(id).update()
        }
        async fn delete_by_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> _toasty::codegen_support::Result<()> {
            self.filter_by_id(id).delete().exec(executor).await
        }
        fn filter_by_id(
            self,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> UserQuery {
            self.filter(User::fields().id().eq(id))
        }
        async fn get_by_tenant_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> _toasty::codegen_support::Result<User> {
            self.filter_by_tenant_id(tenant_id).get(executor).await
        }
        fn update_by_tenant_id(
            self,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> UserUpdate {
            self.filter_by_tenant_id(tenant_id).update()
        }
        async fn delete_by_tenant_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> _toasty::codegen_support::Result<()> {
            self.filter_by_tenant_id(tenant_id).delete().exec(executor).await
        }
        fn filter_by_tenant_id(
            self,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> UserQuery {
            self.filter(User::fields().tenant_id().eq(tenant_id))
        }
        async fn exec(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
        ) -> _toasty::codegen_support::Result<Vec<User>> {
            executor.exec(self.stmt.into()).await
        }
        fn first(self) -> _toasty::codegen_support::stmt::Query<Option<User>> {
            self.stmt.first()
        }
        fn one(self) -> _toasty::codegen_support::stmt::Query<User> {
            self.stmt.one()
        }
        async fn get(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
        ) -> _toasty::codegen_support::Result<User> {
            self.one().exec(executor).await
        }
        fn update(self) -> UserUpdate {
            UserUpdate::from(self)
        }
        fn count(self) -> _toasty::codegen_support::stmt::Query<u64> {
            self.stmt.count()
        }
        fn select<__E, __T>(
            self,
            projection: __E,
        ) -> _toasty::codegen_support::stmt::Query<_toasty::codegen_support::List<__T>>
        where
            __E: _toasty::codegen_support::IntoExpr<__T>,
            __T: _toasty::codegen_support::Load,
        {
            self.stmt.select(projection)
        }
        fn delete(self) -> _toasty::codegen_support::stmt::Delete<()> {
            self.stmt.delete()
        }
        fn paginate(
            self,
            per_page: usize,
        ) -> _toasty::codegen_support::stmt::Paginate<User> {
            _toasty::codegen_support::stmt::Paginate::new(self.stmt, per_page)
        }
        fn filter(self, expr: _toasty::codegen_support::stmt::Expr<bool>) -> UserQuery {
            UserQuery {
                stmt: self.stmt.and(expr),
            }
        }
        fn order_by(
            mut self,
            order_by: impl Into<_toasty::codegen_support::stmt::OrderBy>,
        ) -> UserQuery {
            self.stmt.order_by(order_by);
            self
        }
        fn latest_by<__toasty_T>(
            mut self,
            field: _toasty::codegen_support::stmt::Path<User, __toasty_T>,
        ) -> UserQuery {
            self.stmt.latest_by(field);
            self
        }
        fn limit(mut self, n: usize) -> UserQuery {
            self.stmt.limit(n);
            self
        }
        fn offset(mut self, n: usize) -> UserQuery {
            self.stmt.offset(n);
            self
        }
        fn include<__toasty_T>(
            mut self,
            path: impl _toasty::codegen_support::Into<
                _toasty::codegen_support::Path<User, __toasty_T>,
            >,
        ) -> UserQuery {
            self.stmt.include(path.into());
            self
        }
        fn tenant(
            mut self,
        ) -> <toasty::BelongsTo<Tenant> as _toasty::codegen_support::Relation>::Query {
            use _toasty::codegen_support::IntoStatement;
            <toasty::BelongsTo<
                Tenant,
            > as _toasty::codegen_support::Relation>::Query::from_stmt(
                _toasty::codegen_support::stmt::Association::many_via_one(
                        self.stmt,
                        User::fields().tenant().into(),
                    )
                    .into_statement()
                    .into_query()
                    .unwrap(),
            )
        }
        fn todos(
            mut self,
        ) -> <toasty::HasMany<Todo> as _toasty::codegen_support::Relation>::Query {
            use _toasty::codegen_support::IntoStatement;
            <toasty::HasMany<
                Todo,
            > as _toasty::codegen_support::Relation>::Query::from_stmt(
                _toasty::codegen_support::stmt::Association::many(
                        self.stmt,
                        User::fields().todos().into(),
                    )
                    .into_statement()
                    .into_query()
                    .unwrap(),
            )
        }
    }
    impl _toasty::codegen_support::IntoStatement for UserQuery {
        type Returning = _toasty::codegen_support::List<User>;
        fn into_statement(
            self,
        ) -> _toasty::codegen_support::Statement<_toasty::codegen_support::List<User>> {
            use _toasty::codegen_support::IntoStatement;
            self.stmt.into_statement()
        }
    }
    impl _toasty::codegen_support::IntoStatement for &UserQuery {
        type Returning = _toasty::codegen_support::List<User>;
        fn into_statement(
            self,
        ) -> _toasty::codegen_support::Statement<_toasty::codegen_support::List<User>> {
            use _toasty::codegen_support::IntoStatement;
            self.stmt.clone().into_statement()
        }
    }
    impl _toasty::codegen_support::stmt::IntoScope<User> for UserQuery {
        fn into_scope(
            self,
        ) -> _toasty::codegen_support::Statement<_toasty::codegen_support::List<User>> {
            use _toasty::codegen_support::stmt::IntoScope;
            self.stmt.into_scope()
        }
    }
    impl _toasty::codegen_support::stmt::IntoScope<User> for &UserQuery {
        fn into_scope(
            self,
        ) -> _toasty::codegen_support::Statement<_toasty::codegen_support::List<User>> {
            use _toasty::codegen_support::stmt::IntoScope;
            self.stmt.clone().into_scope()
        }
    }
    impl _toasty::codegen_support::Default for UserQuery {
        fn default() -> UserQuery {
            UserQuery {
                stmt: _toasty::codegen_support::stmt::Query::all(),
            }
        }
    }
    struct UserCreate {
        stmt: _toasty::codegen_support::stmt::Insert<User>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for UserCreate {
        #[inline]
        fn clone(&self) -> UserCreate {
            UserCreate {
                stmt: ::core::clone::Clone::clone(&self.stmt),
            }
        }
    }
    impl UserCreate {
        fn id(
            mut self,
            id: impl _toasty::codegen_support::IntoExpr<
                <String as _toasty::codegen_support::Field>::ExprTarget,
            >,
        ) -> Self {
            self.stmt.set(0, id.into_expr());
            self
        }
        fn tenant_id(
            mut self,
            tenant_id: impl _toasty::codegen_support::IntoExpr<
                <uuid::Uuid as _toasty::codegen_support::Field>::ExprTarget,
            >,
        ) -> Self {
            self.stmt.set(1, tenant_id.into_expr());
            self
        }
        fn tenant(
            mut self,
            tenant: impl _toasty::codegen_support::IntoExpr<
                <toasty::BelongsTo<Tenant> as _toasty::codegen_support::Relation>::Expr,
            >,
        ) -> Self {
            if false {
                let m = <User as _toasty::codegen_support::Load>::load(
                        Default::default(),
                    )
                    .unwrap();
                let _ = &m.tenant;
            }
            self.stmt.set(2, tenant.into_expr());
            self
        }
        fn name(
            mut self,
            name: impl _toasty::codegen_support::IntoExpr<
                <String as _toasty::codegen_support::Field>::ExprTarget,
            >,
        ) -> Self {
            self.stmt.set(3, name.into_expr());
            self
        }
        fn todo(
            mut self,
            todo: impl _toasty::codegen_support::IntoExpr<
                <toasty::HasMany<Todo> as _toasty::codegen_support::Relation>::Expr,
            >,
        ) -> Self {
            self.stmt.insert(4, todo.into_expr());
            self
        }
        fn todos(
            mut self,
            todos: impl _toasty::codegen_support::IntoExpr<
                _toasty::codegen_support::List<
                    <toasty::HasMany<Todo> as _toasty::codegen_support::Relation>::Model,
                >,
            >,
        ) -> Self {
            self.stmt.insert_all(4, todos.into_expr());
            self
        }
        async fn exec(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
        ) -> _toasty::codegen_support::Result<User> {
            executor.exec(self.stmt.into()).await
        }
    }
    impl _toasty::codegen_support::IntoInsert for UserCreate {
        type Model = User;
        fn into_insert(self) -> _toasty::codegen_support::stmt::Insert<User> {
            self.stmt
        }
    }
    impl _toasty::codegen_support::IntoStatement for UserCreate {
        type Returning = User;
        fn into_statement(self) -> _toasty::codegen_support::Statement<User> {
            self.stmt.into()
        }
    }
    impl _toasty::codegen_support::IntoExpr<User> for UserCreate {
        fn into_expr(self) -> _toasty::codegen_support::stmt::Expr<User> {
            self.stmt.into()
        }
        fn by_ref(&self) -> _toasty::codegen_support::stmt::Expr<User> {
            ::core::panicking::panic("not yet implemented")
        }
    }
    impl _toasty::codegen_support::IntoExpr<Option<User>> for UserCreate {
        fn into_expr(self) -> _toasty::codegen_support::stmt::Expr<Option<User>> {
            self.stmt.into()
        }
        fn by_ref(&self) -> _toasty::codegen_support::stmt::Expr<Option<User>> {
            ::core::panicking::panic("not yet implemented")
        }
    }
    impl _toasty::codegen_support::Assign<User> for UserCreate {
        fn into_assignment(self) -> _toasty::codegen_support::stmt::Assignment<User> {
            _toasty::codegen_support::stmt::set(
                <Self as _toasty::codegen_support::IntoExpr<User>>::into_expr(self),
            )
        }
    }
    impl _toasty::codegen_support::Assign<Option<User>> for UserCreate {
        fn into_assignment(
            self,
        ) -> _toasty::codegen_support::stmt::Assignment<Option<User>> {
            _toasty::codegen_support::stmt::set(
                <Self as _toasty::codegen_support::IntoExpr<
                    Option<User>,
                >>::into_expr(self),
            )
        }
    }
    impl Default for UserCreate {
        fn default() -> UserCreate {
            let mut s = UserCreate {
                stmt: _toasty::codegen_support::stmt::Insert::blank_single(),
            };
            s
        }
    }
    struct UserUpdate<__toasty_T: _toasty::codegen_support::UpdateTarget = UserQuery> {
        assignments: _toasty::codegen_support::core::stmt::Assignments,
        condition: Option<_toasty::codegen_support::core::stmt::Expr>,
        target: __toasty_T,
    }
    #[automatically_derived]
    impl<
        __toasty_T: ::core::clone::Clone + _toasty::codegen_support::UpdateTarget,
    > ::core::clone::Clone for UserUpdate<__toasty_T> {
        #[inline]
        fn clone(&self) -> UserUpdate<__toasty_T> {
            UserUpdate {
                assignments: ::core::clone::Clone::clone(&self.assignments),
                condition: ::core::clone::Clone::clone(&self.condition),
                target: ::core::clone::Clone::clone(&self.target),
            }
        }
    }
    impl<__toasty_T: _toasty::codegen_support::UpdateTarget> UserUpdate<__toasty_T> {
        fn apply_update_defaults(&mut self) {}
        fn id(
            mut self,
            id: impl _toasty::codegen_support::Assign<
                <String as _toasty::codegen_support::Field>::ExprTarget,
            >,
        ) -> Self {
            self.set_id(id);
            self
        }
        fn set_id(
            &mut self,
            id: impl _toasty::codegen_support::Assign<
                <String as _toasty::codegen_support::Field>::ExprTarget,
            >,
        ) -> &mut Self {
            let projection = _toasty::codegen_support::stmt::Projection::from_index(0);
            id.assign(&mut self.assignments, projection);
            self
        }
        fn tenant_id(
            mut self,
            tenant_id: impl _toasty::codegen_support::Assign<
                <uuid::Uuid as _toasty::codegen_support::Field>::ExprTarget,
            >,
        ) -> Self {
            self.set_tenant_id(tenant_id);
            self
        }
        fn set_tenant_id(
            &mut self,
            tenant_id: impl _toasty::codegen_support::Assign<
                <uuid::Uuid as _toasty::codegen_support::Field>::ExprTarget,
            >,
        ) -> &mut Self {
            let projection = _toasty::codegen_support::stmt::Projection::from_index(1);
            tenant_id.assign(&mut self.assignments, projection);
            self
        }
        fn tenant(
            mut self,
            tenant: impl _toasty::codegen_support::Assign<
                <toasty::BelongsTo<Tenant> as _toasty::codegen_support::Relation>::Expr,
            >,
        ) -> Self {
            self.set_tenant(tenant);
            self
        }
        fn set_tenant(
            &mut self,
            tenant: impl _toasty::codegen_support::Assign<
                <toasty::BelongsTo<Tenant> as _toasty::codegen_support::Relation>::Expr,
            >,
        ) -> &mut Self {
            let projection = _toasty::codegen_support::stmt::Projection::from_index(2);
            tenant.assign(&mut self.assignments, projection);
            self
        }
        fn name(
            mut self,
            name: impl _toasty::codegen_support::Assign<
                <String as _toasty::codegen_support::Field>::ExprTarget,
            >,
        ) -> Self {
            self.set_name(name);
            self
        }
        fn set_name(
            &mut self,
            name: impl _toasty::codegen_support::Assign<
                <String as _toasty::codegen_support::Field>::ExprTarget,
            >,
        ) -> &mut Self {
            let projection = _toasty::codegen_support::stmt::Projection::from_index(3);
            name.assign(&mut self.assignments, projection);
            self
        }
        fn todos(
            mut self,
            todos: impl _toasty::codegen_support::Assign<
                _toasty::codegen_support::List<
                    <toasty::HasMany<Todo> as _toasty::codegen_support::Relation>::Expr,
                >,
            >,
        ) -> Self {
            self.set_todos(todos);
            self
        }
        fn set_todos(
            &mut self,
            todos: impl _toasty::codegen_support::Assign<
                _toasty::codegen_support::List<
                    <toasty::HasMany<Todo> as _toasty::codegen_support::Relation>::Expr,
                >,
            >,
        ) -> &mut Self {
            let projection = _toasty::codegen_support::stmt::Projection::from_index(4);
            todos.assign(&mut self.assignments, projection);
            self
        }
        fn build_stmt(&mut self) -> _toasty::codegen_support::core::stmt::Statement {
            use _toasty::codegen_support::UpdateTarget as _;
            let assignments = ::std::mem::take(&mut self.assignments);
            let mut stmt = self.target.to_update_stmt(assignments);
            if let Some(cond) = self.condition.take() {
                stmt.as_untyped_mut().condition = _toasty::codegen_support::core::stmt::Condition::new(
                    cond,
                );
            }
            stmt.into_untyped()
        }
        async fn exec(
            mut self,
            executor: &mut dyn _toasty::codegen_support::Executor,
        ) -> _toasty::codegen_support::Result<()> {
            let response = executor.exec_untyped(self.build_stmt()).await?;
            let value = response.values.collect_as_value().await?;
            self.target.apply_result(value)?;
            Ok(())
        }
    }
    impl _toasty::codegen_support::UpdateTarget for &mut User {
        type Returning = User;
        fn to_update_stmt(
            &mut self,
            assignments: _toasty::codegen_support::core::stmt::Assignments,
        ) -> _toasty::codegen_support::stmt::Update<User> {
            use _toasty::codegen_support::IntoStatement;
            let mut stmt = _toasty::codegen_support::stmt::Update::new(
                (&**self).into_statement().into_query().unwrap(),
            );
            stmt.set_assignments(assignments);
            stmt
        }
        fn apply_result(
            self,
            value: _toasty::codegen_support::core::stmt::Value,
        ) -> _toasty::codegen_support::Result<()> {
            <User as _toasty::codegen_support::Load>::reload(self, value)
        }
    }
    impl _toasty::codegen_support::UpdateTarget for UserQuery {
        type Returning = _toasty::codegen_support::List<User>;
        fn to_update_stmt(
            &mut self,
            assignments: _toasty::codegen_support::core::stmt::Assignments,
        ) -> _toasty::codegen_support::stmt::Update<
            _toasty::codegen_support::List<User>,
        > {
            let query = ::std::mem::replace(
                &mut self.stmt,
                _toasty::codegen_support::stmt::Query::all(),
            );
            let mut stmt = _toasty::codegen_support::stmt::Update::new(query);
            stmt.set_assignments(assignments);
            stmt
        }
        fn apply_result(
            self,
            _values: _toasty::codegen_support::core::stmt::Value,
        ) -> _toasty::codegen_support::Result<()> {
            Ok(())
        }
    }
    impl From<UserQuery> for UserUpdate {
        fn from(value: UserQuery) -> UserUpdate {
            let mut s = UserUpdate {
                assignments: _toasty::codegen_support::core::stmt::Assignments::default(),
                condition: None,
                target: value,
            };
            s.apply_update_defaults();
            s
        }
    }
    impl From<
        _toasty::codegen_support::stmt::Query<_toasty::codegen_support::List<User>>,
    > for UserUpdate {
        fn from(
            src: _toasty::codegen_support::stmt::Query<
                _toasty::codegen_support::List<User>,
            >,
        ) -> UserUpdate {
            let mut s = UserUpdate {
                assignments: _toasty::codegen_support::core::stmt::Assignments::default(),
                condition: None,
                target: UserQuery::from_stmt(src),
            };
            s.apply_update_defaults();
            s
        }
    }
    impl _toasty::codegen_support::IntoStatement for UserUpdate {
        type Returning = ();
        fn into_statement(mut self) -> _toasty::codegen_support::Statement<()> {
            _toasty::codegen_support::Statement::from_untyped_stmt(self.build_stmt())
        }
    }
    struct Many {
        stmt: _toasty::codegen_support::stmt::Association<
            _toasty::codegen_support::List<User>,
        >,
    }
    struct One {
        stmt: _toasty::codegen_support::stmt::Query<User>,
    }
    struct OptionOne {
        stmt: _toasty::codegen_support::stmt::Query<
            _toasty::codegen_support::Option<User>,
        >,
    }
    impl Many {
        pub fn from_stmt(
            stmt: _toasty::codegen_support::stmt::Association<
                _toasty::codegen_support::List<User>,
            >,
        ) -> Many {
            Many { stmt }
        }
        async fn get_by_tenant_id_and_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> _toasty::codegen_support::Result<User> {
            self.filter_by_tenant_id_and_id(tenant_id, id).get(executor).await
        }
        fn update_by_tenant_id_and_id(
            self,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> UserUpdate {
            self.filter_by_tenant_id_and_id(tenant_id, id).update()
        }
        async fn delete_by_tenant_id_and_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> _toasty::codegen_support::Result<()> {
            self.filter_by_tenant_id_and_id(tenant_id, id).delete().exec(executor).await
        }
        fn filter_by_tenant_id_and_id(
            self,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> UserQuery {
            UserQuery::from_stmt({
                    use _toasty::codegen_support::IntoStatement;
                    self.into_statement().into_query().unwrap()
                })
                .filter(
                    _toasty::codegen_support::stmt::Expr::and_all([
                        User::fields().tenant_id().eq(tenant_id),
                        User::fields().id().eq(id),
                    ]),
                )
        }
        async fn get_by_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> _toasty::codegen_support::Result<User> {
            self.filter_by_id(id).get(executor).await
        }
        fn update_by_id(
            self,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> UserUpdate {
            self.filter_by_id(id).update()
        }
        async fn delete_by_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> _toasty::codegen_support::Result<()> {
            self.filter_by_id(id).delete().exec(executor).await
        }
        fn filter_by_id(
            self,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> UserQuery {
            UserQuery::from_stmt({
                    use _toasty::codegen_support::IntoStatement;
                    self.into_statement().into_query().unwrap()
                })
                .filter(User::fields().id().eq(id))
        }
        async fn get_by_tenant_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> _toasty::codegen_support::Result<User> {
            self.filter_by_tenant_id(tenant_id).get(executor).await
        }
        fn update_by_tenant_id(
            self,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> UserUpdate {
            self.filter_by_tenant_id(tenant_id).update()
        }
        async fn delete_by_tenant_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> _toasty::codegen_support::Result<()> {
            self.filter_by_tenant_id(tenant_id).delete().exec(executor).await
        }
        fn filter_by_tenant_id(
            self,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> UserQuery {
            UserQuery::from_stmt({
                    use _toasty::codegen_support::IntoStatement;
                    self.into_statement().into_query().unwrap()
                })
                .filter(User::fields().tenant_id().eq(tenant_id))
        }
        fn tenant(
            self,
        ) -> <toasty::BelongsTo<Tenant> as _toasty::codegen_support::Relation>::Many {
            <toasty::BelongsTo<
                Tenant,
            > as _toasty::codegen_support::Relation>::Many::from_stmt(
                self.stmt.chain_field(2),
            )
        }
        fn todos(
            self,
        ) -> <toasty::HasMany<Todo> as _toasty::codegen_support::Relation>::Many {
            <toasty::HasMany<
                Todo,
            > as _toasty::codegen_support::Relation>::Many::from_stmt(
                self.stmt.chain_field(4),
            )
        }
        /// Iterate all entries in the relation
        async fn exec(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
        ) -> _toasty::codegen_support::Result<Vec<User>> {
            use _toasty::codegen_support::IntoStatement;
            self.into_statement().exec(executor).await
        }
        fn filter(
            self,
            filter: _toasty::codegen_support::stmt::Expr<bool>,
        ) -> UserQuery {
            use _toasty::codegen_support::IntoStatement;
            let select = self.into_statement().into_query().unwrap();
            UserQuery::from_stmt(select.and(filter))
        }
        fn create(self) -> UserCreate {
            let mut builder = UserCreate::default();
            builder.stmt.set_scope(self.stmt);
            builder
        }
        /// Add an item to the association
        async fn insert(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            item: impl _toasty::codegen_support::IntoExpr<User>,
        ) -> _toasty::codegen_support::Result<()> {
            executor.exec(self.stmt.insert(item)).await
        }
        /// Remove items from the association
        async fn remove(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            item: impl _toasty::codegen_support::IntoExpr<User>,
        ) -> _toasty::codegen_support::Result<()> {
            executor.exec(self.stmt.remove(item)).await
        }
    }
    impl _toasty::codegen_support::IntoStatement for Many {
        type Returning = _toasty::codegen_support::List<User>;
        fn into_statement(
            self,
        ) -> _toasty::codegen_support::Statement<_toasty::codegen_support::List<User>> {
            use _toasty::codegen_support::IntoStatement;
            self.stmt.into_statement()
        }
    }
    impl One {
        fn from_stmt(
            stmt: _toasty::codegen_support::stmt::Query<
                _toasty::codegen_support::List<User>,
            >,
        ) -> One {
            One { stmt: stmt.one() }
        }
        /// Create a new associated record
        fn create(self) -> UserCreate {
            let mut builder = UserCreate::default();
            builder.stmt.set_scope(self.stmt);
            builder
        }
        async fn exec(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
        ) -> _toasty::codegen_support::Result<User> {
            self.stmt.exec(executor).await
        }
    }
    impl _toasty::codegen_support::IntoStatement for One {
        type Returning = User;
        fn into_statement(self) -> _toasty::codegen_support::Statement<User> {
            use _toasty::codegen_support::IntoStatement;
            self.stmt.into_statement()
        }
    }
    impl OptionOne {
        pub fn from_stmt(
            stmt: _toasty::codegen_support::stmt::Query<
                _toasty::codegen_support::List<User>,
            >,
        ) -> OptionOne {
            OptionOne { stmt: stmt.first() }
        }
        /// Create a new associated record
        fn create(self) -> UserCreate {
            let mut builder = UserCreate::default();
            builder.stmt.set_scope(self.stmt);
            builder
        }
        async fn exec(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
        ) -> _toasty::codegen_support::Result<_toasty::codegen_support::Option<User>> {
            self.stmt.exec(executor).await
        }
    }
    #[diagnostic::do_not_recommend]
    impl _toasty::codegen_support::Scope for Many {
        type Item = _toasty::codegen_support::List<User>;
        type Path<__Origin> = UserListFields<__Origin>;
        type Create = UserCreate;
        fn new_path<__Origin>(
            path: _toasty::codegen_support::Path<__Origin, Self::Item>,
        ) -> Self::Path<__Origin> {
            UserListFields::from_path(path)
        }
        fn new_create() -> Self::Create {
            UserCreate::default()
        }
        fn new_path_root() -> Self::Path<Self::Item> {
            UserListFields::from_path(_toasty::codegen_support::Path::from_model_list())
        }
    }
    #[diagnostic::do_not_recommend]
    impl _toasty::codegen_support::Scope for One {
        type Item = User;
        type Path<__Origin> = UserFields<__Origin>;
        type Create = UserCreate;
        fn new_path<__Origin>(
            path: _toasty::codegen_support::Path<__Origin, Self::Item>,
        ) -> Self::Path<__Origin> {
            UserFields::from_path(path)
        }
        fn new_create() -> Self::Create {
            UserCreate::default()
        }
        fn new_path_root() -> Self::Path<Self::Item> {
            UserFields::from_path(_toasty::codegen_support::Path::root())
        }
    }
    #[diagnostic::do_not_recommend]
    impl _toasty::codegen_support::Scope for OptionOne {
        type Item = User;
        type Path<__Origin> = UserFields<__Origin>;
        type Create = UserCreate;
        fn new_path<__Origin>(
            path: _toasty::codegen_support::Path<__Origin, Self::Item>,
        ) -> Self::Path<__Origin> {
            UserFields::from_path(path)
        }
        fn new_create() -> Self::Create {
            UserCreate::default()
        }
        fn new_path_root() -> Self::Path<Self::Item> {
            UserFields::from_path(_toasty::codegen_support::Path::root())
        }
    }
    #[diagnostic::do_not_recommend]
    impl _toasty::codegen_support::ValidateCreate for Many {
        const CREATE_META: &'static _toasty::codegen_support::CreateMeta = &<User as _toasty::codegen_support::Model>::CREATE_META;
    }
    #[diagnostic::do_not_recommend]
    impl _toasty::codegen_support::ValidateCreate for One {
        const CREATE_META: &'static _toasty::codegen_support::CreateMeta = &<User as _toasty::codegen_support::Model>::CREATE_META;
    }
    #[diagnostic::do_not_recommend]
    impl _toasty::codegen_support::ValidateCreate for OptionOne {
        const CREATE_META: &'static _toasty::codegen_support::CreateMeta = &<User as _toasty::codegen_support::Model>::CREATE_META;
    }
    #[diagnostic::do_not_recommend]
    impl<__Origin> _toasty::codegen_support::ValidateCreate for UserFields<__Origin> {
        const CREATE_META: &'static _toasty::codegen_support::CreateMeta = &<User as _toasty::codegen_support::Model>::CREATE_META;
    }
    #[diagnostic::do_not_recommend]
    impl<__Origin> _toasty::codegen_support::ValidateCreate
    for UserListFields<__Origin> {
        const CREATE_META: &'static _toasty::codegen_support::CreateMeta = &<User as _toasty::codegen_support::Model>::CREATE_META;
    }
};
#[item_collection(User)]
#[key(partition = tenant_id, local = [user_id, id])]
#[index(tenant_id, user_id)]
struct Todo {
    id: String,
    tenant_id: uuid::Uuid,
    user_id: String,
    #[belongs_to(key = [tenant_id, user_id], references = [tenant_id, id])]
    user: toasty::BelongsTo<User>,
    title: String,
}
#[automatically_derived]
impl ::core::fmt::Debug for Todo {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field5_finish(
            f,
            "Todo",
            "id",
            &self.id,
            "tenant_id",
            &self.tenant_id,
            "user_id",
            &self.user_id,
            "user",
            &self.user,
            "title",
            &&self.title,
        )
    }
}
const _: () = {
    use toasty as _toasty;
    impl Todo {
        fn fields() -> TodoFields<Todo> {
            TodoFields {
                path: _toasty::codegen_support::Path::root(),
            }
        }
        async fn get_by_tenant_id_and_user_id_and_id(
            executor: &mut dyn _toasty::codegen_support::Executor,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> _toasty::codegen_support::Result<Todo> {
            Self::filter_by_tenant_id_and_user_id_and_id(tenant_id, user_id, id)
                .get(executor)
                .await
        }
        fn update_by_tenant_id_and_user_id_and_id(
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> TodoUpdate {
            Self::filter_by_tenant_id_and_user_id_and_id(tenant_id, user_id, id).update()
        }
        async fn delete_by_tenant_id_and_user_id_and_id(
            executor: &mut dyn _toasty::codegen_support::Executor,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> _toasty::codegen_support::Result<()> {
            Self::filter_by_tenant_id_and_user_id_and_id(tenant_id, user_id, id)
                .delete()
                .exec(executor)
                .await
        }
        fn filter_by_tenant_id_and_user_id_and_id(
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> TodoQuery {
            TodoQuery::default()
                .filter_by_tenant_id_and_user_id_and_id(tenant_id, user_id, id)
        }
        async fn get_by_tenant_id_and_user_id(
            executor: &mut dyn _toasty::codegen_support::Executor,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> _toasty::codegen_support::Result<Todo> {
            Self::filter_by_tenant_id_and_user_id(tenant_id, user_id).get(executor).await
        }
        fn update_by_tenant_id_and_user_id(
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> TodoUpdate {
            Self::filter_by_tenant_id_and_user_id(tenant_id, user_id).update()
        }
        async fn delete_by_tenant_id_and_user_id(
            executor: &mut dyn _toasty::codegen_support::Executor,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> _toasty::codegen_support::Result<()> {
            Self::filter_by_tenant_id_and_user_id(tenant_id, user_id)
                .delete()
                .exec(executor)
                .await
        }
        fn filter_by_tenant_id_and_user_id(
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> TodoQuery {
            TodoQuery::default().filter_by_tenant_id_and_user_id(tenant_id, user_id)
        }
        async fn get_by_tenant_id(
            executor: &mut dyn _toasty::codegen_support::Executor,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> _toasty::codegen_support::Result<Todo> {
            Self::filter_by_tenant_id(tenant_id).get(executor).await
        }
        fn update_by_tenant_id(
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> TodoUpdate {
            Self::filter_by_tenant_id(tenant_id).update()
        }
        async fn delete_by_tenant_id(
            executor: &mut dyn _toasty::codegen_support::Executor,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> _toasty::codegen_support::Result<()> {
            Self::filter_by_tenant_id(tenant_id).delete().exec(executor).await
        }
        fn filter_by_tenant_id(
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> TodoQuery {
            TodoQuery::default().filter_by_tenant_id(tenant_id)
        }
        fn user(
            &self,
        ) -> <toasty::BelongsTo<User> as _toasty::codegen_support::Relation>::One {
            if false {
                let _ = &self.user;
            }
            {
                use _toasty::codegen_support::IntoStatement;
                <toasty::BelongsTo<
                    User,
                > as _toasty::codegen_support::Relation>::One::from_stmt(
                    <toasty::BelongsTo<
                        User,
                    > as _toasty::codegen_support::Relation>::Model::filter(
                            _toasty::codegen_support::stmt::Expr::and_all([
                                _toasty::codegen_support::Field::key_constraint(
                                    &self.tenant_id,
                                    <toasty::BelongsTo<
                                        User,
                                    > as _toasty::codegen_support::Relation>::Model::fields()
                                        .tenant_id()
                                        .into(),
                                ),
                                _toasty::codegen_support::Field::key_constraint(
                                    &self.user_id,
                                    <toasty::BelongsTo<
                                        User,
                                    > as _toasty::codegen_support::Relation>::Model::fields()
                                        .id()
                                        .into(),
                                ),
                            ]),
                        )
                        .into_statement()
                        .into_query()
                        .unwrap(),
                )
            }
        }
        #[doc(hidden)]
        fn verify_pair_belongs_to_exists_for_user(&self) -> &toasty::BelongsTo<User> {
            let _ = &self.tenant_id;
            let _ = &self.user_id;
            &self.user
        }
        #[doc(hidden)]
        pub const fn __check_create_fields(__provided: &[&str]) {
            if !<String as _toasty::codegen_support::Field>::NULLABLE
                && !_toasty::codegen_support::const_contains(__provided, "id")
            {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("missing required field `id` in create! for `Todo`"),
                    );
                };
            }
            if !<String as _toasty::codegen_support::Field>::NULLABLE
                && !_toasty::codegen_support::const_contains(__provided, "title")
            {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "missing required field `title` in create! for `Todo`",
                        ),
                    );
                };
            }
        }
        fn create() -> TodoCreate {
            TodoCreate::default()
        }
        fn create_many() -> _toasty::codegen_support::stmt::CreateMany<Todo> {
            _toasty::codegen_support::stmt::CreateMany::default()
        }
        fn update(&mut self) -> TodoUpdate<&mut Self> {
            let mut s = TodoUpdate {
                assignments: _toasty::codegen_support::core::stmt::Assignments::default(),
                condition: None,
                target: self,
            };
            s.apply_update_defaults();
            s
        }
        fn all() -> TodoQuery {
            TodoQuery::default()
        }
        fn filter(expr: _toasty::codegen_support::stmt::Expr<bool>) -> TodoQuery {
            TodoQuery::from_stmt(_toasty::codegen_support::stmt::Query::filter(expr))
        }
        fn delete(self) -> _toasty::codegen_support::stmt::Delete<()> {
            {
                let __delete = TodoQuery::default()
                    .filter_by_tenant_id_and_user_id_and_id(
                        &self.tenant_id,
                        &self.user_id,
                        &self.id,
                    )
                    .delete();
                __delete
            }
        }
    }
    impl _toasty::codegen_support::Register for Todo {
        fn id() -> _toasty::codegen_support::core::schema::app::ModelId {
            static ID: std::sync::OnceLock<
                _toasty::codegen_support::core::schema::app::ModelId,
            > = std::sync::OnceLock::new();
            *ID.get_or_init(|| _toasty::codegen_support::generate_unique_id())
        }
        fn schema() -> _toasty::codegen_support::core::schema::app::Model {
            let id = Todo::id();
            _toasty::codegen_support::core::schema::app::Model::Root(_toasty::codegen_support::core::schema::app::ModelRoot {
                id,
                name: _toasty::codegen_support::core::schema::Name {
                    parts: ::alloc::boxed::box_assume_init_into_vec_unsafe(
                        ::alloc::intrinsics::write_box_via_move(
                            ::alloc::boxed::Box::new_uninit(),
                            ["todo".to_string()],
                        ),
                    ),
                },
                fields: ::alloc::boxed::box_assume_init_into_vec_unsafe(
                    ::alloc::intrinsics::write_box_via_move(
                        ::alloc::boxed::Box::new_uninit(),
                        [
                            _toasty::codegen_support::core::schema::app::Field {
                                id: _toasty::codegen_support::core::schema::app::FieldId {
                                    model: Todo::id(),
                                    index: 0,
                                },
                                name: _toasty::codegen_support::core::schema::app::FieldName {
                                    app: Some("id".to_string()),
                                    storage: None,
                                },
                                ty: <String as _toasty::codegen_support::Field>::field_ty(
                                    None,
                                ),
                                nullable: <String as _toasty::codegen_support::Field>::NULLABLE,
                                primary_key: true,
                                auto: None,
                                versionable: false,
                                deferred: false,
                                constraints: ::alloc::vec::Vec::new(),
                                variant: None,
                            },
                            _toasty::codegen_support::core::schema::app::Field {
                                id: _toasty::codegen_support::core::schema::app::FieldId {
                                    model: Todo::id(),
                                    index: 1,
                                },
                                name: _toasty::codegen_support::core::schema::app::FieldName {
                                    app: Some("tenant_id".to_string()),
                                    storage: None,
                                },
                                ty: <uuid::Uuid as _toasty::codegen_support::Field>::field_ty(
                                    None,
                                ),
                                nullable: <uuid::Uuid as _toasty::codegen_support::Field>::NULLABLE,
                                primary_key: true,
                                auto: None,
                                versionable: false,
                                deferred: false,
                                constraints: ::alloc::vec::Vec::new(),
                                variant: None,
                            },
                            _toasty::codegen_support::core::schema::app::Field {
                                id: _toasty::codegen_support::core::schema::app::FieldId {
                                    model: Todo::id(),
                                    index: 2,
                                },
                                name: _toasty::codegen_support::core::schema::app::FieldName {
                                    app: Some("user_id".to_string()),
                                    storage: None,
                                },
                                ty: <String as _toasty::codegen_support::Field>::field_ty(
                                    None,
                                ),
                                nullable: <String as _toasty::codegen_support::Field>::NULLABLE,
                                primary_key: true,
                                auto: None,
                                versionable: false,
                                deferred: false,
                                constraints: ::alloc::vec::Vec::new(),
                                variant: None,
                            },
                            _toasty::codegen_support::core::schema::app::Field {
                                id: _toasty::codegen_support::core::schema::app::FieldId {
                                    model: Todo::id(),
                                    index: 3,
                                },
                                name: _toasty::codegen_support::core::schema::app::FieldName {
                                    app: Some("user".to_string()),
                                    storage: None,
                                },
                                ty: <toasty::BelongsTo<
                                    User,
                                > as _toasty::codegen_support::Relation>::belongs_to_field_ty(_toasty::codegen_support::core::schema::app::ForeignKey {
                                    fields: ::alloc::boxed::box_assume_init_into_vec_unsafe(
                                        ::alloc::intrinsics::write_box_via_move(
                                            ::alloc::boxed::Box::new_uninit(),
                                            [
                                                _toasty::codegen_support::core::schema::app::ForeignKeyField {
                                                    source: _toasty::codegen_support::core::schema::app::FieldId {
                                                        model: Todo::id(),
                                                        index: 1,
                                                    },
                                                    target: <toasty::BelongsTo<
                                                        User,
                                                    > as _toasty::codegen_support::Relation>::field_name_to_id(
                                                        "tenant_id",
                                                    ),
                                                },
                                                _toasty::codegen_support::core::schema::app::ForeignKeyField {
                                                    source: _toasty::codegen_support::core::schema::app::FieldId {
                                                        model: Todo::id(),
                                                        index: 2,
                                                    },
                                                    target: <toasty::BelongsTo<
                                                        User,
                                                    > as _toasty::codegen_support::Relation>::field_name_to_id(
                                                        "id",
                                                    ),
                                                },
                                            ],
                                        ),
                                    ),
                                }),
                                nullable: <toasty::BelongsTo<
                                    User,
                                > as _toasty::codegen_support::Relation>::nullable(),
                                primary_key: false,
                                auto: None,
                                versionable: false,
                                deferred: false,
                                constraints: ::alloc::vec::Vec::new(),
                                variant: None,
                            },
                            _toasty::codegen_support::core::schema::app::Field {
                                id: _toasty::codegen_support::core::schema::app::FieldId {
                                    model: Todo::id(),
                                    index: 4,
                                },
                                name: _toasty::codegen_support::core::schema::app::FieldName {
                                    app: Some("title".to_string()),
                                    storage: None,
                                },
                                ty: <String as _toasty::codegen_support::Field>::field_ty(
                                    None,
                                ),
                                nullable: <String as _toasty::codegen_support::Field>::NULLABLE,
                                primary_key: false,
                                auto: None,
                                versionable: false,
                                deferred: false,
                                constraints: ::alloc::vec::Vec::new(),
                                variant: None,
                            },
                        ],
                    ),
                ),
                primary_key: _toasty::codegen_support::core::schema::app::PrimaryKey {
                    fields: ::alloc::boxed::box_assume_init_into_vec_unsafe(
                        ::alloc::intrinsics::write_box_via_move(
                            ::alloc::boxed::Box::new_uninit(),
                            [
                                _toasty::codegen_support::core::schema::app::FieldId {
                                    model: id,
                                    index: 1,
                                },
                                _toasty::codegen_support::core::schema::app::FieldId {
                                    model: id,
                                    index: 2,
                                },
                                _toasty::codegen_support::core::schema::app::FieldId {
                                    model: id,
                                    index: 0,
                                },
                            ],
                        ),
                    ),
                    index: _toasty::codegen_support::core::schema::app::IndexId {
                        model: id,
                        index: 0,
                    },
                },
                table_name: None,
                item_collection: Some(
                    <User as _toasty::codegen_support::Register>::id(),
                ),
                indices: ::alloc::boxed::box_assume_init_into_vec_unsafe(
                    ::alloc::intrinsics::write_box_via_move(
                        ::alloc::boxed::Box::new_uninit(),
                        [
                            _toasty::codegen_support::core::schema::app::Index {
                                id: _toasty::codegen_support::core::schema::app::IndexId {
                                    model: id,
                                    index: 0,
                                },
                                name: None,
                                fields: ::alloc::boxed::box_assume_init_into_vec_unsafe(
                                    ::alloc::intrinsics::write_box_via_move(
                                        ::alloc::boxed::Box::new_uninit(),
                                        [
                                            _toasty::codegen_support::core::schema::app::IndexField {
                                                field: _toasty::codegen_support::core::schema::app::FieldId {
                                                    model: id,
                                                    index: 1,
                                                },
                                                op: _toasty::codegen_support::core::schema::db::IndexOp::Eq,
                                                scope: _toasty::codegen_support::core::schema::db::IndexScope::Partition,
                                            },
                                            _toasty::codegen_support::core::schema::app::IndexField {
                                                field: _toasty::codegen_support::core::schema::app::FieldId {
                                                    model: id,
                                                    index: 2,
                                                },
                                                op: _toasty::codegen_support::core::schema::db::IndexOp::Eq,
                                                scope: _toasty::codegen_support::core::schema::db::IndexScope::Local,
                                            },
                                            _toasty::codegen_support::core::schema::app::IndexField {
                                                field: _toasty::codegen_support::core::schema::app::FieldId {
                                                    model: id,
                                                    index: 0,
                                                },
                                                op: _toasty::codegen_support::core::schema::db::IndexOp::Eq,
                                                scope: _toasty::codegen_support::core::schema::db::IndexScope::Local,
                                            },
                                        ],
                                    ),
                                ),
                                unique: true,
                                primary_key: true,
                            },
                            _toasty::codegen_support::core::schema::app::Index {
                                id: _toasty::codegen_support::core::schema::app::IndexId {
                                    model: id,
                                    index: 1,
                                },
                                name: None,
                                fields: ::alloc::boxed::box_assume_init_into_vec_unsafe(
                                    ::alloc::intrinsics::write_box_via_move(
                                        ::alloc::boxed::Box::new_uninit(),
                                        [
                                            _toasty::codegen_support::core::schema::app::IndexField {
                                                field: _toasty::codegen_support::core::schema::app::FieldId {
                                                    model: id,
                                                    index: 1,
                                                },
                                                op: _toasty::codegen_support::core::schema::db::IndexOp::Eq,
                                                scope: _toasty::codegen_support::core::schema::db::IndexScope::Partition,
                                            },
                                            _toasty::codegen_support::core::schema::app::IndexField {
                                                field: _toasty::codegen_support::core::schema::app::FieldId {
                                                    model: id,
                                                    index: 2,
                                                },
                                                op: _toasty::codegen_support::core::schema::db::IndexOp::Eq,
                                                scope: _toasty::codegen_support::core::schema::db::IndexScope::Local,
                                            },
                                        ],
                                    ),
                                ),
                                unique: false,
                                primary_key: false,
                            },
                        ],
                    ),
                ),
                version_field: None,
            })
        }
        fn register(
            model_set: &mut _toasty::codegen_support::core::schema::app::ModelSet,
        ) {
            if model_set.contains(Self::id()) {
                return;
            }
            model_set.add(Self::schema());
            <String as _toasty::codegen_support::Field>::register(model_set);
            <uuid::Uuid as _toasty::codegen_support::Field>::register(model_set);
            <String as _toasty::codegen_support::Field>::register(model_set);
            <<toasty::BelongsTo<
                User,
            > as _toasty::codegen_support::Relation>::Model as _toasty::codegen_support::Register>::register(
                model_set,
            );
            <String as _toasty::codegen_support::Field>::register(model_set);
        }
    }
    #[allow(non_upper_case_globals)]
    const _: () = {
        static __INVENTORY: ::inventory::Node = ::inventory::Node {
            value: &{
                _toasty::codegen_support::DiscoverItem::new(
                    "example-item-collection",
                    |model_set| {
                        <Todo as _toasty::codegen_support::Register>::register(
                            model_set,
                        );
                    },
                )
            },
            next: ::inventory::__private::UnsafeCell::new(
                ::inventory::__private::Option::None,
            ),
        };
        #[link_section = ".text.startup"]
        unsafe extern "C" fn __ctor() {
            unsafe { ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY) }
        }
        #[used]
        #[link_section = ".init_array"]
        static __CTOR: unsafe extern "C" fn() = __ctor;
    };
    impl _toasty::codegen_support::Load for Todo {
        type Output = Self;
        fn ty() -> _toasty::codegen_support::core::stmt::Type {
            _toasty::codegen_support::core::stmt::Type::Model(
                <Self as _toasty::codegen_support::Register>::id(),
            )
        }
        fn load(
            value: _toasty::codegen_support::core::stmt::Value,
        ) -> _toasty::codegen_support::Result<Self> {
            match value {
                _toasty::codegen_support::core::stmt::Value::Null => {
                    Err(_toasty::codegen_support::Error::record_not_found("Todo"))
                }
                _toasty::codegen_support::core::stmt::Value::Record(mut record) => {
                    Ok(Todo {
                        id: <String as _toasty::codegen_support::Load>::load(
                            record[0].take(),
                        )?,
                        tenant_id: <uuid::Uuid as _toasty::codegen_support::Load>::load(
                            record[1].take(),
                        )?,
                        user_id: <String as _toasty::codegen_support::Load>::load(
                            record[2].take(),
                        )?,
                        user: _toasty::codegen_support::BelongsTo::load(
                            record[3usize].take(),
                        )?,
                        title: <String as _toasty::codegen_support::Load>::load(
                            record[4].take(),
                        )?,
                    })
                }
                value => {
                    Err(_toasty::codegen_support::Error::type_conversion(value, "Todo"))
                }
            }
        }
        fn reload(
            target: &mut Self,
            value: _toasty::codegen_support::core::stmt::Value,
        ) -> _toasty::codegen_support::Result<()> {
            for (field, value) in value.into_sparse_record().into_iter() {
                match field {
                    0 => {
                        <String as _toasty::codegen_support::Load>::reload(
                            &mut target.id,
                            value,
                        )?
                    }
                    1 => {
                        <uuid::Uuid as _toasty::codegen_support::Load>::reload(
                            &mut target.tenant_id,
                            value,
                        )?
                    }
                    2 => {
                        <String as _toasty::codegen_support::Load>::reload(
                            &mut target.user_id,
                            value,
                        )?
                    }
                    3 => target.user.unload(),
                    4 => {
                        <String as _toasty::codegen_support::Load>::reload(
                            &mut target.title,
                            value,
                        )?
                    }
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "not yet implemented: {0}",
                                format_args!(
                                    "handle unknown field id in reload after update",
                                ),
                            ),
                        );
                    }
                }
            }
            Ok(())
        }
    }
    impl _toasty::codegen_support::Model for Todo {
        type Query = TodoQuery;
        type Create = TodoCreate;
        type Update<'a> = TodoUpdate<&'a mut Self>;
        type UpdateQuery = TodoUpdate;
        type Path<__Origin> = TodoFields<__Origin>;
        type PrimaryKey = (uuid::Uuid, String, String);
        const CREATE_META: _toasty::codegen_support::CreateMeta = _toasty::codegen_support::CreateMeta {
            fields: &[
                _toasty::codegen_support::CreateField {
                    name: "id",
                    required: !<String as _toasty::codegen_support::Field>::NULLABLE,
                },
                _toasty::codegen_support::CreateField {
                    name: "title",
                    required: !<String as _toasty::codegen_support::Field>::NULLABLE,
                },
            ],
            model_name: "Todo",
        };
        fn new_path<__Origin>(
            path: _toasty::codegen_support::Path<__Origin, Self>,
        ) -> Self::Path<__Origin> {
            TodoFields::from_path(path)
        }
        fn find_by_primary_key(
            id: _toasty::codegen_support::stmt::Expr<Self::PrimaryKey>,
        ) -> Self::Query {
            let pk_expr: _toasty::codegen_support::stmt::Expr<Self::PrimaryKey> = _toasty::codegen_support::IntoExpr::into_expr((
                Todo::fields().tenant_id(),
                Todo::fields().user_id(),
                Todo::fields().id(),
            ));
            Self::filter(pk_expr.eq(id))
        }
    }
    impl _toasty::codegen_support::Relation for Todo {
        type Model = Todo;
        type Expr = Todo;
        type Query = TodoQuery;
        type Create = TodoCreate;
        type Many = Many;
        type ManyField<__Origin> = TodoListFields<__Origin>;
        type One = One;
        type OneField<__Origin> = TodoFields<__Origin>;
        type OptionOne = OptionOne;
        fn new_many_field<__Origin>(
            path: _toasty::codegen_support::Path<
                __Origin,
                _toasty::codegen_support::List<Self::Model>,
            >,
        ) -> TodoListFields<__Origin> {
            TodoListFields::from_path(path)
        }
        fn field_name_to_id(
            name: &str,
        ) -> _toasty::codegen_support::core::schema::app::FieldId {
            use _toasty::codegen_support::{Model, Register};
            match name {
                "id" => {
                    _toasty::codegen_support::core::schema::app::FieldId {
                        model: Self::id(),
                        index: 0,
                    }
                }
                "tenant_id" => {
                    _toasty::codegen_support::core::schema::app::FieldId {
                        model: Self::id(),
                        index: 1,
                    }
                }
                "user_id" => {
                    _toasty::codegen_support::core::schema::app::FieldId {
                        model: Self::id(),
                        index: 2,
                    }
                }
                "user" => {
                    _toasty::codegen_support::core::schema::app::FieldId {
                        model: Self::id(),
                        index: 3,
                    }
                }
                "title" => {
                    _toasty::codegen_support::core::schema::app::FieldId {
                        model: Self::id(),
                        index: 4,
                    }
                }
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "not yet implemented: {0}",
                            format_args!("field_name_to_id: {0}", name),
                        ),
                    );
                }
            }
        }
    }
    impl _toasty::codegen_support::stmt::IntoExpr<Todo> for Todo {
        fn into_expr(self) -> _toasty::codegen_support::stmt::Expr<Todo> {
            let expr: _toasty::codegen_support::stmt::Expr<
                (uuid::Uuid, String, String),
            > = _toasty::codegen_support::IntoExpr::into_expr((
                self.tenant_id,
                self.user_id,
                self.id,
            ));
            expr.cast()
        }
        fn by_ref(&self) -> _toasty::codegen_support::stmt::Expr<Todo> {
            let expr: _toasty::codegen_support::stmt::Expr<
                (uuid::Uuid, String, String),
            > = _toasty::codegen_support::IntoExpr::into_expr((
                &self.tenant_id,
                &self.user_id,
                &self.id,
            ));
            expr.cast()
        }
    }
    impl _toasty::codegen_support::Assign<Todo> for Todo {
        fn into_assignment(self) -> _toasty::codegen_support::stmt::Assignment<Todo> {
            _toasty::codegen_support::stmt::set(
                <Self as _toasty::codegen_support::IntoExpr<Todo>>::into_expr(self),
            )
        }
    }
    impl _toasty::codegen_support::IntoStatement for &Todo {
        type Returning = Todo;
        fn into_statement(self) -> _toasty::codegen_support::Statement<Todo> {
            use _toasty::codegen_support::IntoStatement;
            _toasty::codegen_support::IntoStatement::into_statement(
                TodoQuery::default()
                    .filter_by_tenant_id_and_user_id_and_id(
                        &self.tenant_id,
                        &self.user_id,
                        &self.id,
                    )
                    .one(),
            )
        }
    }
    impl _toasty::codegen_support::IntoStatement for &mut Todo {
        type Returning = Todo;
        fn into_statement(self) -> _toasty::codegen_support::Statement<Todo> {
            (&*self).into_statement()
        }
    }
    impl _toasty::codegen_support::IntoStatement for Todo {
        type Returning = Todo;
        fn into_statement(self) -> _toasty::codegen_support::Statement<Todo> {
            (&self).into_statement()
        }
    }
    struct TodoFields<__Origin> {
        path: _toasty::codegen_support::Path<__Origin, Todo>,
    }
    impl<__Origin> TodoFields<__Origin> {
        const fn from_path(
            path: _toasty::codegen_support::Path<__Origin, Todo>,
        ) -> TodoFields<__Origin> {
            TodoFields { path }
        }
        fn path(&self) -> _toasty::codegen_support::Path<__Origin, Todo> {
            self.path.clone()
        }
        fn eq(
            self,
            rhs: impl _toasty::codegen_support::IntoExpr<Todo>,
        ) -> _toasty::codegen_support::stmt::Expr<bool> {
            use _toasty::codegen_support::IntoExpr;
            self.path.eq(rhs.into_expr())
        }
        fn in_query(
            self,
            rhs: impl _toasty::codegen_support::IntoStatement<
                Returning = _toasty::codegen_support::List<Todo>,
            >,
        ) -> _toasty::codegen_support::stmt::Expr<bool> {
            self.path.in_query(rhs)
        }
        fn create(&self) -> TodoCreate {
            TodoCreate::default()
        }
        fn id(&self) -> <String as _toasty::codegen_support::Field>::Path<__Origin> {
            <String as _toasty::codegen_support::Field>::new_path(
                self
                    .path()
                    .chain(
                        _toasty::codegen_support::Path::<
                            Todo,
                            <String as _toasty::codegen_support::Field>::ExprTarget,
                        >::from_field_index(0),
                    ),
            )
        }
        fn tenant_id(
            &self,
        ) -> <uuid::Uuid as _toasty::codegen_support::Field>::Path<__Origin> {
            <uuid::Uuid as _toasty::codegen_support::Field>::new_path(
                self
                    .path()
                    .chain(
                        _toasty::codegen_support::Path::<
                            Todo,
                            <uuid::Uuid as _toasty::codegen_support::Field>::ExprTarget,
                        >::from_field_index(1),
                    ),
            )
        }
        fn user_id(
            &self,
        ) -> <String as _toasty::codegen_support::Field>::Path<__Origin> {
            <String as _toasty::codegen_support::Field>::new_path(
                self
                    .path()
                    .chain(
                        _toasty::codegen_support::Path::<
                            Todo,
                            <String as _toasty::codegen_support::Field>::ExprTarget,
                        >::from_field_index(2),
                    ),
            )
        }
        fn user(
            &self,
        ) -> <toasty::BelongsTo<
            User,
        > as _toasty::codegen_support::Relation>::OneField<__Origin> {
            <toasty::BelongsTo<
                User,
            > as _toasty::codegen_support::Relation>::OneField::from_path(
                self
                    .path()
                    .chain(
                        _toasty::codegen_support::Path::<Todo, _>::from_field_index(3),
                    ),
            )
        }
        fn title(&self) -> <String as _toasty::codegen_support::Field>::Path<__Origin> {
            <String as _toasty::codegen_support::Field>::new_path(
                self
                    .path()
                    .chain(
                        _toasty::codegen_support::Path::<
                            Todo,
                            <String as _toasty::codegen_support::Field>::ExprTarget,
                        >::from_field_index(4),
                    ),
            )
        }
    }
    impl<__Origin> Into<_toasty::codegen_support::Path<__Origin, Todo>>
    for TodoFields<__Origin> {
        fn into(self) -> _toasty::codegen_support::Path<__Origin, Todo> {
            self.path
        }
    }
    impl<__Origin> _toasty::codegen_support::IntoExpr<Todo> for TodoFields<__Origin> {
        fn into_expr(self) -> _toasty::codegen_support::stmt::Expr<Todo> {
            self.path.into_expr()
        }
        fn by_ref(&self) -> _toasty::codegen_support::stmt::Expr<Todo> {
            self.path.by_ref()
        }
    }
    struct TodoListFields<__Origin> {
        path: _toasty::codegen_support::Path<
            __Origin,
            _toasty::codegen_support::List<Todo>,
        >,
    }
    impl<__Origin> TodoListFields<__Origin> {
        const fn from_path(
            path: _toasty::codegen_support::Path<
                __Origin,
                _toasty::codegen_support::List<Todo>,
            >,
        ) -> TodoListFields<__Origin> {
            TodoListFields { path }
        }
        fn path(
            &self,
        ) -> _toasty::codegen_support::Path<
            __Origin,
            _toasty::codegen_support::List<Todo>,
        > {
            self.path.clone()
        }
        /// Filter the parent model by a condition on the associated
        /// (child) model. Returns `true` when **any** associated record
        /// satisfies `filter`.
        fn any(
            self,
            filter: _toasty::codegen_support::stmt::Expr<bool>,
        ) -> _toasty::codegen_support::stmt::Expr<bool> {
            self.path.any(filter)
        }
        /// Filter the parent model by a condition on the associated
        /// (child) model. Returns `true` when **all** associated records
        /// satisfy `filter` (vacuously true when there are no
        /// associated records).
        fn all(
            self,
            filter: _toasty::codegen_support::stmt::Expr<bool>,
        ) -> _toasty::codegen_support::stmt::Expr<bool> {
            self.path.all(filter)
        }
        fn create(&self) -> TodoCreate {
            TodoCreate::default()
        }
        fn id(&self) -> <String as _toasty::codegen_support::Field>::ListPath<__Origin> {
            <String as _toasty::codegen_support::Field>::new_list_path(
                self
                    .path()
                    .chain(
                        _toasty::codegen_support::Path::<Todo, _>::from_field_index(0),
                    ),
            )
        }
        fn tenant_id(
            &self,
        ) -> <uuid::Uuid as _toasty::codegen_support::Field>::ListPath<__Origin> {
            <uuid::Uuid as _toasty::codegen_support::Field>::new_list_path(
                self
                    .path()
                    .chain(
                        _toasty::codegen_support::Path::<Todo, _>::from_field_index(1),
                    ),
            )
        }
        fn user_id(
            &self,
        ) -> <String as _toasty::codegen_support::Field>::ListPath<__Origin> {
            <String as _toasty::codegen_support::Field>::new_list_path(
                self
                    .path()
                    .chain(
                        _toasty::codegen_support::Path::<Todo, _>::from_field_index(2),
                    ),
            )
        }
        fn user(
            &self,
        ) -> <toasty::BelongsTo<
            User,
        > as _toasty::codegen_support::Relation>::ManyField<__Origin> {
            <toasty::BelongsTo<
                User,
            > as _toasty::codegen_support::Relation>::ManyField::from_path(
                self
                    .path()
                    .chain(
                        _toasty::codegen_support::Path::<Todo, _>::from_field_index(3),
                    ),
            )
        }
        fn title(
            &self,
        ) -> <String as _toasty::codegen_support::Field>::ListPath<__Origin> {
            <String as _toasty::codegen_support::Field>::new_list_path(
                self
                    .path()
                    .chain(
                        _toasty::codegen_support::Path::<Todo, _>::from_field_index(4),
                    ),
            )
        }
    }
    impl<
        __Origin,
    > Into<
        _toasty::codegen_support::Path<__Origin, _toasty::codegen_support::List<Todo>>,
    > for TodoListFields<__Origin> {
        fn into(
            self,
        ) -> _toasty::codegen_support::Path<
            __Origin,
            _toasty::codegen_support::List<Todo>,
        > {
            self.path
        }
    }
    impl<
        __Origin,
    > _toasty::codegen_support::IntoExpr<_toasty::codegen_support::List<Todo>>
    for TodoListFields<__Origin> {
        fn into_expr(
            self,
        ) -> _toasty::codegen_support::stmt::Expr<_toasty::codegen_support::List<Todo>> {
            self.path.into_expr()
        }
        fn by_ref(
            &self,
        ) -> _toasty::codegen_support::stmt::Expr<_toasty::codegen_support::List<Todo>> {
            self.path.by_ref()
        }
    }
    struct TodoQuery {
        stmt: _toasty::codegen_support::stmt::Query<
            _toasty::codegen_support::List<Todo>,
        >,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TodoQuery {
        #[inline]
        fn clone(&self) -> TodoQuery {
            TodoQuery {
                stmt: ::core::clone::Clone::clone(&self.stmt),
            }
        }
    }
    impl TodoQuery {
        const fn from_stmt(
            stmt: _toasty::codegen_support::stmt::Query<
                _toasty::codegen_support::List<Todo>,
            >,
        ) -> TodoQuery {
            TodoQuery { stmt }
        }
        async fn get_by_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> _toasty::codegen_support::Result<Todo> {
            self.filter_by_id(id).get(executor).await
        }
        fn update_by_id(
            self,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> TodoUpdate {
            self.filter_by_id(id).update()
        }
        async fn delete_by_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> _toasty::codegen_support::Result<()> {
            self.filter_by_id(id).delete().exec(executor).await
        }
        fn filter_by_id(
            self,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> TodoQuery {
            self.filter(Todo::fields().id().eq(id))
        }
        async fn get_by_user_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> _toasty::codegen_support::Result<Todo> {
            self.filter_by_user_id(user_id).get(executor).await
        }
        fn update_by_user_id(
            self,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> TodoUpdate {
            self.filter_by_user_id(user_id).update()
        }
        async fn delete_by_user_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> _toasty::codegen_support::Result<()> {
            self.filter_by_user_id(user_id).delete().exec(executor).await
        }
        fn filter_by_user_id(
            self,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> TodoQuery {
            self.filter(Todo::fields().user_id().eq(user_id))
        }
        async fn get_by_tenant_id_and_user_id_and_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> _toasty::codegen_support::Result<Todo> {
            self.filter_by_tenant_id_and_user_id_and_id(tenant_id, user_id, id)
                .get(executor)
                .await
        }
        fn update_by_tenant_id_and_user_id_and_id(
            self,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> TodoUpdate {
            self.filter_by_tenant_id_and_user_id_and_id(tenant_id, user_id, id).update()
        }
        async fn delete_by_tenant_id_and_user_id_and_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> _toasty::codegen_support::Result<()> {
            self.filter_by_tenant_id_and_user_id_and_id(tenant_id, user_id, id)
                .delete()
                .exec(executor)
                .await
        }
        fn filter_by_tenant_id_and_user_id_and_id(
            self,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> TodoQuery {
            self.filter(
                _toasty::codegen_support::stmt::Expr::and_all([
                    Todo::fields().tenant_id().eq(tenant_id),
                    Todo::fields().user_id().eq(user_id),
                    Todo::fields().id().eq(id),
                ]),
            )
        }
        async fn get_by_tenant_id_and_user_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> _toasty::codegen_support::Result<Todo> {
            self.filter_by_tenant_id_and_user_id(tenant_id, user_id).get(executor).await
        }
        fn update_by_tenant_id_and_user_id(
            self,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> TodoUpdate {
            self.filter_by_tenant_id_and_user_id(tenant_id, user_id).update()
        }
        async fn delete_by_tenant_id_and_user_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> _toasty::codegen_support::Result<()> {
            self.filter_by_tenant_id_and_user_id(tenant_id, user_id)
                .delete()
                .exec(executor)
                .await
        }
        fn filter_by_tenant_id_and_user_id(
            self,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> TodoQuery {
            self.filter(
                _toasty::codegen_support::stmt::Expr::and_all([
                    Todo::fields().tenant_id().eq(tenant_id),
                    Todo::fields().user_id().eq(user_id),
                ]),
            )
        }
        async fn get_by_tenant_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> _toasty::codegen_support::Result<Todo> {
            self.filter_by_tenant_id(tenant_id).get(executor).await
        }
        fn update_by_tenant_id(
            self,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> TodoUpdate {
            self.filter_by_tenant_id(tenant_id).update()
        }
        async fn delete_by_tenant_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> _toasty::codegen_support::Result<()> {
            self.filter_by_tenant_id(tenant_id).delete().exec(executor).await
        }
        fn filter_by_tenant_id(
            self,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> TodoQuery {
            self.filter(Todo::fields().tenant_id().eq(tenant_id))
        }
        async fn get_by_user_id_and_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> _toasty::codegen_support::Result<Todo> {
            self.filter_by_user_id_and_id(user_id, id).get(executor).await
        }
        fn update_by_user_id_and_id(
            self,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> TodoUpdate {
            self.filter_by_user_id_and_id(user_id, id).update()
        }
        async fn delete_by_user_id_and_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> _toasty::codegen_support::Result<()> {
            self.filter_by_user_id_and_id(user_id, id).delete().exec(executor).await
        }
        fn filter_by_user_id_and_id(
            self,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> TodoQuery {
            self.filter(
                _toasty::codegen_support::stmt::Expr::and_all([
                    Todo::fields().user_id().eq(user_id),
                    Todo::fields().id().eq(id),
                ]),
            )
        }
        async fn exec(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
        ) -> _toasty::codegen_support::Result<Vec<Todo>> {
            executor.exec(self.stmt.into()).await
        }
        fn first(self) -> _toasty::codegen_support::stmt::Query<Option<Todo>> {
            self.stmt.first()
        }
        fn one(self) -> _toasty::codegen_support::stmt::Query<Todo> {
            self.stmt.one()
        }
        async fn get(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
        ) -> _toasty::codegen_support::Result<Todo> {
            self.one().exec(executor).await
        }
        fn update(self) -> TodoUpdate {
            TodoUpdate::from(self)
        }
        fn count(self) -> _toasty::codegen_support::stmt::Query<u64> {
            self.stmt.count()
        }
        fn select<__E, __T>(
            self,
            projection: __E,
        ) -> _toasty::codegen_support::stmt::Query<_toasty::codegen_support::List<__T>>
        where
            __E: _toasty::codegen_support::IntoExpr<__T>,
            __T: _toasty::codegen_support::Load,
        {
            self.stmt.select(projection)
        }
        fn delete(self) -> _toasty::codegen_support::stmt::Delete<()> {
            self.stmt.delete()
        }
        fn paginate(
            self,
            per_page: usize,
        ) -> _toasty::codegen_support::stmt::Paginate<Todo> {
            _toasty::codegen_support::stmt::Paginate::new(self.stmt, per_page)
        }
        fn filter(self, expr: _toasty::codegen_support::stmt::Expr<bool>) -> TodoQuery {
            TodoQuery {
                stmt: self.stmt.and(expr),
            }
        }
        fn order_by(
            mut self,
            order_by: impl Into<_toasty::codegen_support::stmt::OrderBy>,
        ) -> TodoQuery {
            self.stmt.order_by(order_by);
            self
        }
        fn latest_by<__toasty_T>(
            mut self,
            field: _toasty::codegen_support::stmt::Path<Todo, __toasty_T>,
        ) -> TodoQuery {
            self.stmt.latest_by(field);
            self
        }
        fn limit(mut self, n: usize) -> TodoQuery {
            self.stmt.limit(n);
            self
        }
        fn offset(mut self, n: usize) -> TodoQuery {
            self.stmt.offset(n);
            self
        }
        fn include<__toasty_T>(
            mut self,
            path: impl _toasty::codegen_support::Into<
                _toasty::codegen_support::Path<Todo, __toasty_T>,
            >,
        ) -> TodoQuery {
            self.stmt.include(path.into());
            self
        }
        fn user(
            mut self,
        ) -> <toasty::BelongsTo<User> as _toasty::codegen_support::Relation>::Query {
            use _toasty::codegen_support::IntoStatement;
            <toasty::BelongsTo<
                User,
            > as _toasty::codegen_support::Relation>::Query::from_stmt(
                _toasty::codegen_support::stmt::Association::many_via_one(
                        self.stmt,
                        Todo::fields().user().into(),
                    )
                    .into_statement()
                    .into_query()
                    .unwrap(),
            )
        }
    }
    impl _toasty::codegen_support::IntoStatement for TodoQuery {
        type Returning = _toasty::codegen_support::List<Todo>;
        fn into_statement(
            self,
        ) -> _toasty::codegen_support::Statement<_toasty::codegen_support::List<Todo>> {
            use _toasty::codegen_support::IntoStatement;
            self.stmt.into_statement()
        }
    }
    impl _toasty::codegen_support::IntoStatement for &TodoQuery {
        type Returning = _toasty::codegen_support::List<Todo>;
        fn into_statement(
            self,
        ) -> _toasty::codegen_support::Statement<_toasty::codegen_support::List<Todo>> {
            use _toasty::codegen_support::IntoStatement;
            self.stmt.clone().into_statement()
        }
    }
    impl _toasty::codegen_support::stmt::IntoScope<Todo> for TodoQuery {
        fn into_scope(
            self,
        ) -> _toasty::codegen_support::Statement<_toasty::codegen_support::List<Todo>> {
            use _toasty::codegen_support::stmt::IntoScope;
            self.stmt.into_scope()
        }
    }
    impl _toasty::codegen_support::stmt::IntoScope<Todo> for &TodoQuery {
        fn into_scope(
            self,
        ) -> _toasty::codegen_support::Statement<_toasty::codegen_support::List<Todo>> {
            use _toasty::codegen_support::stmt::IntoScope;
            self.stmt.clone().into_scope()
        }
    }
    impl _toasty::codegen_support::Default for TodoQuery {
        fn default() -> TodoQuery {
            TodoQuery {
                stmt: _toasty::codegen_support::stmt::Query::all(),
            }
        }
    }
    struct TodoCreate {
        stmt: _toasty::codegen_support::stmt::Insert<Todo>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TodoCreate {
        #[inline]
        fn clone(&self) -> TodoCreate {
            TodoCreate {
                stmt: ::core::clone::Clone::clone(&self.stmt),
            }
        }
    }
    impl TodoCreate {
        fn id(
            mut self,
            id: impl _toasty::codegen_support::IntoExpr<
                <String as _toasty::codegen_support::Field>::ExprTarget,
            >,
        ) -> Self {
            self.stmt.set(0, id.into_expr());
            self
        }
        fn tenant_id(
            mut self,
            tenant_id: impl _toasty::codegen_support::IntoExpr<
                <uuid::Uuid as _toasty::codegen_support::Field>::ExprTarget,
            >,
        ) -> Self {
            self.stmt.set(1, tenant_id.into_expr());
            self
        }
        fn user_id(
            mut self,
            user_id: impl _toasty::codegen_support::IntoExpr<
                <String as _toasty::codegen_support::Field>::ExprTarget,
            >,
        ) -> Self {
            self.stmt.set(2, user_id.into_expr());
            self
        }
        fn user(
            mut self,
            user: impl _toasty::codegen_support::IntoExpr<
                <toasty::BelongsTo<User> as _toasty::codegen_support::Relation>::Expr,
            >,
        ) -> Self {
            if false {
                let m = <Todo as _toasty::codegen_support::Load>::load(
                        Default::default(),
                    )
                    .unwrap();
                let _ = &m.user;
            }
            self.stmt.set(3, user.into_expr());
            self
        }
        fn title(
            mut self,
            title: impl _toasty::codegen_support::IntoExpr<
                <String as _toasty::codegen_support::Field>::ExprTarget,
            >,
        ) -> Self {
            self.stmt.set(4, title.into_expr());
            self
        }
        async fn exec(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
        ) -> _toasty::codegen_support::Result<Todo> {
            executor.exec(self.stmt.into()).await
        }
    }
    impl _toasty::codegen_support::IntoInsert for TodoCreate {
        type Model = Todo;
        fn into_insert(self) -> _toasty::codegen_support::stmt::Insert<Todo> {
            self.stmt
        }
    }
    impl _toasty::codegen_support::IntoStatement for TodoCreate {
        type Returning = Todo;
        fn into_statement(self) -> _toasty::codegen_support::Statement<Todo> {
            self.stmt.into()
        }
    }
    impl _toasty::codegen_support::IntoExpr<Todo> for TodoCreate {
        fn into_expr(self) -> _toasty::codegen_support::stmt::Expr<Todo> {
            self.stmt.into()
        }
        fn by_ref(&self) -> _toasty::codegen_support::stmt::Expr<Todo> {
            ::core::panicking::panic("not yet implemented")
        }
    }
    impl _toasty::codegen_support::IntoExpr<Option<Todo>> for TodoCreate {
        fn into_expr(self) -> _toasty::codegen_support::stmt::Expr<Option<Todo>> {
            self.stmt.into()
        }
        fn by_ref(&self) -> _toasty::codegen_support::stmt::Expr<Option<Todo>> {
            ::core::panicking::panic("not yet implemented")
        }
    }
    impl _toasty::codegen_support::Assign<Todo> for TodoCreate {
        fn into_assignment(self) -> _toasty::codegen_support::stmt::Assignment<Todo> {
            _toasty::codegen_support::stmt::set(
                <Self as _toasty::codegen_support::IntoExpr<Todo>>::into_expr(self),
            )
        }
    }
    impl _toasty::codegen_support::Assign<Option<Todo>> for TodoCreate {
        fn into_assignment(
            self,
        ) -> _toasty::codegen_support::stmt::Assignment<Option<Todo>> {
            _toasty::codegen_support::stmt::set(
                <Self as _toasty::codegen_support::IntoExpr<
                    Option<Todo>,
                >>::into_expr(self),
            )
        }
    }
    impl Default for TodoCreate {
        fn default() -> TodoCreate {
            let mut s = TodoCreate {
                stmt: _toasty::codegen_support::stmt::Insert::blank_single(),
            };
            s
        }
    }
    struct TodoUpdate<__toasty_T: _toasty::codegen_support::UpdateTarget = TodoQuery> {
        assignments: _toasty::codegen_support::core::stmt::Assignments,
        condition: Option<_toasty::codegen_support::core::stmt::Expr>,
        target: __toasty_T,
    }
    #[automatically_derived]
    impl<
        __toasty_T: ::core::clone::Clone + _toasty::codegen_support::UpdateTarget,
    > ::core::clone::Clone for TodoUpdate<__toasty_T> {
        #[inline]
        fn clone(&self) -> TodoUpdate<__toasty_T> {
            TodoUpdate {
                assignments: ::core::clone::Clone::clone(&self.assignments),
                condition: ::core::clone::Clone::clone(&self.condition),
                target: ::core::clone::Clone::clone(&self.target),
            }
        }
    }
    impl<__toasty_T: _toasty::codegen_support::UpdateTarget> TodoUpdate<__toasty_T> {
        fn apply_update_defaults(&mut self) {}
        fn id(
            mut self,
            id: impl _toasty::codegen_support::Assign<
                <String as _toasty::codegen_support::Field>::ExprTarget,
            >,
        ) -> Self {
            self.set_id(id);
            self
        }
        fn set_id(
            &mut self,
            id: impl _toasty::codegen_support::Assign<
                <String as _toasty::codegen_support::Field>::ExprTarget,
            >,
        ) -> &mut Self {
            let projection = _toasty::codegen_support::stmt::Projection::from_index(0);
            id.assign(&mut self.assignments, projection);
            self
        }
        fn tenant_id(
            mut self,
            tenant_id: impl _toasty::codegen_support::Assign<
                <uuid::Uuid as _toasty::codegen_support::Field>::ExprTarget,
            >,
        ) -> Self {
            self.set_tenant_id(tenant_id);
            self
        }
        fn set_tenant_id(
            &mut self,
            tenant_id: impl _toasty::codegen_support::Assign<
                <uuid::Uuid as _toasty::codegen_support::Field>::ExprTarget,
            >,
        ) -> &mut Self {
            let projection = _toasty::codegen_support::stmt::Projection::from_index(1);
            tenant_id.assign(&mut self.assignments, projection);
            self
        }
        fn user_id(
            mut self,
            user_id: impl _toasty::codegen_support::Assign<
                <String as _toasty::codegen_support::Field>::ExprTarget,
            >,
        ) -> Self {
            self.set_user_id(user_id);
            self
        }
        fn set_user_id(
            &mut self,
            user_id: impl _toasty::codegen_support::Assign<
                <String as _toasty::codegen_support::Field>::ExprTarget,
            >,
        ) -> &mut Self {
            let projection = _toasty::codegen_support::stmt::Projection::from_index(2);
            user_id.assign(&mut self.assignments, projection);
            self
        }
        fn user(
            mut self,
            user: impl _toasty::codegen_support::Assign<
                <toasty::BelongsTo<User> as _toasty::codegen_support::Relation>::Expr,
            >,
        ) -> Self {
            self.set_user(user);
            self
        }
        fn set_user(
            &mut self,
            user: impl _toasty::codegen_support::Assign<
                <toasty::BelongsTo<User> as _toasty::codegen_support::Relation>::Expr,
            >,
        ) -> &mut Self {
            let projection = _toasty::codegen_support::stmt::Projection::from_index(3);
            user.assign(&mut self.assignments, projection);
            self
        }
        fn title(
            mut self,
            title: impl _toasty::codegen_support::Assign<
                <String as _toasty::codegen_support::Field>::ExprTarget,
            >,
        ) -> Self {
            self.set_title(title);
            self
        }
        fn set_title(
            &mut self,
            title: impl _toasty::codegen_support::Assign<
                <String as _toasty::codegen_support::Field>::ExprTarget,
            >,
        ) -> &mut Self {
            let projection = _toasty::codegen_support::stmt::Projection::from_index(4);
            title.assign(&mut self.assignments, projection);
            self
        }
        fn build_stmt(&mut self) -> _toasty::codegen_support::core::stmt::Statement {
            use _toasty::codegen_support::UpdateTarget as _;
            let assignments = ::std::mem::take(&mut self.assignments);
            let mut stmt = self.target.to_update_stmt(assignments);
            if let Some(cond) = self.condition.take() {
                stmt.as_untyped_mut().condition = _toasty::codegen_support::core::stmt::Condition::new(
                    cond,
                );
            }
            stmt.into_untyped()
        }
        async fn exec(
            mut self,
            executor: &mut dyn _toasty::codegen_support::Executor,
        ) -> _toasty::codegen_support::Result<()> {
            let response = executor.exec_untyped(self.build_stmt()).await?;
            let value = response.values.collect_as_value().await?;
            self.target.apply_result(value)?;
            Ok(())
        }
    }
    impl _toasty::codegen_support::UpdateTarget for &mut Todo {
        type Returning = Todo;
        fn to_update_stmt(
            &mut self,
            assignments: _toasty::codegen_support::core::stmt::Assignments,
        ) -> _toasty::codegen_support::stmt::Update<Todo> {
            use _toasty::codegen_support::IntoStatement;
            let mut stmt = _toasty::codegen_support::stmt::Update::new(
                (&**self).into_statement().into_query().unwrap(),
            );
            stmt.set_assignments(assignments);
            stmt
        }
        fn apply_result(
            self,
            value: _toasty::codegen_support::core::stmt::Value,
        ) -> _toasty::codegen_support::Result<()> {
            <Todo as _toasty::codegen_support::Load>::reload(self, value)
        }
    }
    impl _toasty::codegen_support::UpdateTarget for TodoQuery {
        type Returning = _toasty::codegen_support::List<Todo>;
        fn to_update_stmt(
            &mut self,
            assignments: _toasty::codegen_support::core::stmt::Assignments,
        ) -> _toasty::codegen_support::stmt::Update<
            _toasty::codegen_support::List<Todo>,
        > {
            let query = ::std::mem::replace(
                &mut self.stmt,
                _toasty::codegen_support::stmt::Query::all(),
            );
            let mut stmt = _toasty::codegen_support::stmt::Update::new(query);
            stmt.set_assignments(assignments);
            stmt
        }
        fn apply_result(
            self,
            _values: _toasty::codegen_support::core::stmt::Value,
        ) -> _toasty::codegen_support::Result<()> {
            Ok(())
        }
    }
    impl From<TodoQuery> for TodoUpdate {
        fn from(value: TodoQuery) -> TodoUpdate {
            let mut s = TodoUpdate {
                assignments: _toasty::codegen_support::core::stmt::Assignments::default(),
                condition: None,
                target: value,
            };
            s.apply_update_defaults();
            s
        }
    }
    impl From<
        _toasty::codegen_support::stmt::Query<_toasty::codegen_support::List<Todo>>,
    > for TodoUpdate {
        fn from(
            src: _toasty::codegen_support::stmt::Query<
                _toasty::codegen_support::List<Todo>,
            >,
        ) -> TodoUpdate {
            let mut s = TodoUpdate {
                assignments: _toasty::codegen_support::core::stmt::Assignments::default(),
                condition: None,
                target: TodoQuery::from_stmt(src),
            };
            s.apply_update_defaults();
            s
        }
    }
    impl _toasty::codegen_support::IntoStatement for TodoUpdate {
        type Returning = ();
        fn into_statement(mut self) -> _toasty::codegen_support::Statement<()> {
            _toasty::codegen_support::Statement::from_untyped_stmt(self.build_stmt())
        }
    }
    struct Many {
        stmt: _toasty::codegen_support::stmt::Association<
            _toasty::codegen_support::List<Todo>,
        >,
    }
    struct One {
        stmt: _toasty::codegen_support::stmt::Query<Todo>,
    }
    struct OptionOne {
        stmt: _toasty::codegen_support::stmt::Query<
            _toasty::codegen_support::Option<Todo>,
        >,
    }
    impl Many {
        pub fn from_stmt(
            stmt: _toasty::codegen_support::stmt::Association<
                _toasty::codegen_support::List<Todo>,
            >,
        ) -> Many {
            Many { stmt }
        }
        async fn get_by_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> _toasty::codegen_support::Result<Todo> {
            self.filter_by_id(id).get(executor).await
        }
        fn update_by_id(
            self,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> TodoUpdate {
            self.filter_by_id(id).update()
        }
        async fn delete_by_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> _toasty::codegen_support::Result<()> {
            self.filter_by_id(id).delete().exec(executor).await
        }
        fn filter_by_id(
            self,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> TodoQuery {
            TodoQuery::from_stmt({
                    use _toasty::codegen_support::IntoStatement;
                    self.into_statement().into_query().unwrap()
                })
                .filter(Todo::fields().id().eq(id))
        }
        async fn get_by_user_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> _toasty::codegen_support::Result<Todo> {
            self.filter_by_user_id(user_id).get(executor).await
        }
        fn update_by_user_id(
            self,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> TodoUpdate {
            self.filter_by_user_id(user_id).update()
        }
        async fn delete_by_user_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> _toasty::codegen_support::Result<()> {
            self.filter_by_user_id(user_id).delete().exec(executor).await
        }
        fn filter_by_user_id(
            self,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> TodoQuery {
            TodoQuery::from_stmt({
                    use _toasty::codegen_support::IntoStatement;
                    self.into_statement().into_query().unwrap()
                })
                .filter(Todo::fields().user_id().eq(user_id))
        }
        async fn get_by_tenant_id_and_user_id_and_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> _toasty::codegen_support::Result<Todo> {
            self.filter_by_tenant_id_and_user_id_and_id(tenant_id, user_id, id)
                .get(executor)
                .await
        }
        fn update_by_tenant_id_and_user_id_and_id(
            self,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> TodoUpdate {
            self.filter_by_tenant_id_and_user_id_and_id(tenant_id, user_id, id).update()
        }
        async fn delete_by_tenant_id_and_user_id_and_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> _toasty::codegen_support::Result<()> {
            self.filter_by_tenant_id_and_user_id_and_id(tenant_id, user_id, id)
                .delete()
                .exec(executor)
                .await
        }
        fn filter_by_tenant_id_and_user_id_and_id(
            self,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> TodoQuery {
            TodoQuery::from_stmt({
                    use _toasty::codegen_support::IntoStatement;
                    self.into_statement().into_query().unwrap()
                })
                .filter(
                    _toasty::codegen_support::stmt::Expr::and_all([
                        Todo::fields().tenant_id().eq(tenant_id),
                        Todo::fields().user_id().eq(user_id),
                        Todo::fields().id().eq(id),
                    ]),
                )
        }
        async fn get_by_tenant_id_and_user_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> _toasty::codegen_support::Result<Todo> {
            self.filter_by_tenant_id_and_user_id(tenant_id, user_id).get(executor).await
        }
        fn update_by_tenant_id_and_user_id(
            self,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> TodoUpdate {
            self.filter_by_tenant_id_and_user_id(tenant_id, user_id).update()
        }
        async fn delete_by_tenant_id_and_user_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> _toasty::codegen_support::Result<()> {
            self.filter_by_tenant_id_and_user_id(tenant_id, user_id)
                .delete()
                .exec(executor)
                .await
        }
        fn filter_by_tenant_id_and_user_id(
            self,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> TodoQuery {
            TodoQuery::from_stmt({
                    use _toasty::codegen_support::IntoStatement;
                    self.into_statement().into_query().unwrap()
                })
                .filter(
                    _toasty::codegen_support::stmt::Expr::and_all([
                        Todo::fields().tenant_id().eq(tenant_id),
                        Todo::fields().user_id().eq(user_id),
                    ]),
                )
        }
        async fn get_by_tenant_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> _toasty::codegen_support::Result<Todo> {
            self.filter_by_tenant_id(tenant_id).get(executor).await
        }
        fn update_by_tenant_id(
            self,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> TodoUpdate {
            self.filter_by_tenant_id(tenant_id).update()
        }
        async fn delete_by_tenant_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> _toasty::codegen_support::Result<()> {
            self.filter_by_tenant_id(tenant_id).delete().exec(executor).await
        }
        fn filter_by_tenant_id(
            self,
            tenant_id: impl _toasty::codegen_support::IntoExpr<uuid::Uuid>,
        ) -> TodoQuery {
            TodoQuery::from_stmt({
                    use _toasty::codegen_support::IntoStatement;
                    self.into_statement().into_query().unwrap()
                })
                .filter(Todo::fields().tenant_id().eq(tenant_id))
        }
        async fn get_by_user_id_and_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> _toasty::codegen_support::Result<Todo> {
            self.filter_by_user_id_and_id(user_id, id).get(executor).await
        }
        fn update_by_user_id_and_id(
            self,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> TodoUpdate {
            self.filter_by_user_id_and_id(user_id, id).update()
        }
        async fn delete_by_user_id_and_id(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> _toasty::codegen_support::Result<()> {
            self.filter_by_user_id_and_id(user_id, id).delete().exec(executor).await
        }
        fn filter_by_user_id_and_id(
            self,
            user_id: impl _toasty::codegen_support::IntoExpr<String>,
            id: impl _toasty::codegen_support::IntoExpr<String>,
        ) -> TodoQuery {
            TodoQuery::from_stmt({
                    use _toasty::codegen_support::IntoStatement;
                    self.into_statement().into_query().unwrap()
                })
                .filter(
                    _toasty::codegen_support::stmt::Expr::and_all([
                        Todo::fields().user_id().eq(user_id),
                        Todo::fields().id().eq(id),
                    ]),
                )
        }
        fn user(
            self,
        ) -> <toasty::BelongsTo<User> as _toasty::codegen_support::Relation>::Many {
            <toasty::BelongsTo<
                User,
            > as _toasty::codegen_support::Relation>::Many::from_stmt(
                self.stmt.chain_field(3),
            )
        }
        /// Iterate all entries in the relation
        async fn exec(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
        ) -> _toasty::codegen_support::Result<Vec<Todo>> {
            use _toasty::codegen_support::IntoStatement;
            self.into_statement().exec(executor).await
        }
        fn filter(
            self,
            filter: _toasty::codegen_support::stmt::Expr<bool>,
        ) -> TodoQuery {
            use _toasty::codegen_support::IntoStatement;
            let select = self.into_statement().into_query().unwrap();
            TodoQuery::from_stmt(select.and(filter))
        }
        fn create(self) -> TodoCreate {
            let mut builder = TodoCreate::default();
            builder.stmt.set_scope(self.stmt);
            builder
        }
        /// Add an item to the association
        async fn insert(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            item: impl _toasty::codegen_support::IntoExpr<Todo>,
        ) -> _toasty::codegen_support::Result<()> {
            executor.exec(self.stmt.insert(item)).await
        }
        /// Remove items from the association
        async fn remove(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
            item: impl _toasty::codegen_support::IntoExpr<Todo>,
        ) -> _toasty::codegen_support::Result<()> {
            executor.exec(self.stmt.remove(item)).await
        }
    }
    impl _toasty::codegen_support::IntoStatement for Many {
        type Returning = _toasty::codegen_support::List<Todo>;
        fn into_statement(
            self,
        ) -> _toasty::codegen_support::Statement<_toasty::codegen_support::List<Todo>> {
            use _toasty::codegen_support::IntoStatement;
            self.stmt.into_statement()
        }
    }
    impl One {
        fn from_stmt(
            stmt: _toasty::codegen_support::stmt::Query<
                _toasty::codegen_support::List<Todo>,
            >,
        ) -> One {
            One { stmt: stmt.one() }
        }
        /// Create a new associated record
        fn create(self) -> TodoCreate {
            let mut builder = TodoCreate::default();
            builder.stmt.set_scope(self.stmt);
            builder
        }
        async fn exec(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
        ) -> _toasty::codegen_support::Result<Todo> {
            self.stmt.exec(executor).await
        }
    }
    impl _toasty::codegen_support::IntoStatement for One {
        type Returning = Todo;
        fn into_statement(self) -> _toasty::codegen_support::Statement<Todo> {
            use _toasty::codegen_support::IntoStatement;
            self.stmt.into_statement()
        }
    }
    impl OptionOne {
        pub fn from_stmt(
            stmt: _toasty::codegen_support::stmt::Query<
                _toasty::codegen_support::List<Todo>,
            >,
        ) -> OptionOne {
            OptionOne { stmt: stmt.first() }
        }
        /// Create a new associated record
        fn create(self) -> TodoCreate {
            let mut builder = TodoCreate::default();
            builder.stmt.set_scope(self.stmt);
            builder
        }
        async fn exec(
            self,
            executor: &mut dyn _toasty::codegen_support::Executor,
        ) -> _toasty::codegen_support::Result<_toasty::codegen_support::Option<Todo>> {
            self.stmt.exec(executor).await
        }
    }
    #[diagnostic::do_not_recommend]
    impl _toasty::codegen_support::Scope for Many {
        type Item = _toasty::codegen_support::List<Todo>;
        type Path<__Origin> = TodoListFields<__Origin>;
        type Create = TodoCreate;
        fn new_path<__Origin>(
            path: _toasty::codegen_support::Path<__Origin, Self::Item>,
        ) -> Self::Path<__Origin> {
            TodoListFields::from_path(path)
        }
        fn new_create() -> Self::Create {
            TodoCreate::default()
        }
        fn new_path_root() -> Self::Path<Self::Item> {
            TodoListFields::from_path(_toasty::codegen_support::Path::from_model_list())
        }
    }
    #[diagnostic::do_not_recommend]
    impl _toasty::codegen_support::Scope for One {
        type Item = Todo;
        type Path<__Origin> = TodoFields<__Origin>;
        type Create = TodoCreate;
        fn new_path<__Origin>(
            path: _toasty::codegen_support::Path<__Origin, Self::Item>,
        ) -> Self::Path<__Origin> {
            TodoFields::from_path(path)
        }
        fn new_create() -> Self::Create {
            TodoCreate::default()
        }
        fn new_path_root() -> Self::Path<Self::Item> {
            TodoFields::from_path(_toasty::codegen_support::Path::root())
        }
    }
    #[diagnostic::do_not_recommend]
    impl _toasty::codegen_support::Scope for OptionOne {
        type Item = Todo;
        type Path<__Origin> = TodoFields<__Origin>;
        type Create = TodoCreate;
        fn new_path<__Origin>(
            path: _toasty::codegen_support::Path<__Origin, Self::Item>,
        ) -> Self::Path<__Origin> {
            TodoFields::from_path(path)
        }
        fn new_create() -> Self::Create {
            TodoCreate::default()
        }
        fn new_path_root() -> Self::Path<Self::Item> {
            TodoFields::from_path(_toasty::codegen_support::Path::root())
        }
    }
    #[diagnostic::do_not_recommend]
    impl _toasty::codegen_support::ValidateCreate for Many {
        const CREATE_META: &'static _toasty::codegen_support::CreateMeta = &<Todo as _toasty::codegen_support::Model>::CREATE_META;
    }
    #[diagnostic::do_not_recommend]
    impl _toasty::codegen_support::ValidateCreate for One {
        const CREATE_META: &'static _toasty::codegen_support::CreateMeta = &<Todo as _toasty::codegen_support::Model>::CREATE_META;
    }
    #[diagnostic::do_not_recommend]
    impl _toasty::codegen_support::ValidateCreate for OptionOne {
        const CREATE_META: &'static _toasty::codegen_support::CreateMeta = &<Todo as _toasty::codegen_support::Model>::CREATE_META;
    }
    #[diagnostic::do_not_recommend]
    impl<__Origin> _toasty::codegen_support::ValidateCreate for TodoFields<__Origin> {
        const CREATE_META: &'static _toasty::codegen_support::CreateMeta = &<Todo as _toasty::codegen_support::Model>::CREATE_META;
    }
    #[diagnostic::do_not_recommend]
    impl<__Origin> _toasty::codegen_support::ValidateCreate
    for TodoListFields<__Origin> {
        const CREATE_META: &'static _toasty::codegen_support::CreateMeta = &<Todo as _toasty::codegen_support::Model>::CREATE_META;
    }
};
fn main() -> toasty::Result<()> {
    let body = async {
        let mut db = toasty::Db::builder()
            .models({
                let mut model_set = ::toasty::schema::ModelSet::new();
                {
                    <Tenant as ::toasty::schema::Register>::register(&mut model_set);
                    {
                        <User as ::toasty::schema::Register>::register(&mut model_set);
                        {
                            <Todo as ::toasty::schema::Register>::register(
                                &mut model_set,
                            );
                        };
                    };
                };
                model_set
            })
            .connect(
                std::env::var("TOASTY_CONNECTION_URL")
                    .as_deref()
                    .unwrap_or("sqlite::memory:"),
            )
            .await?;
        db.push_schema().await?;
        let acme = {
            const _CREATE: () = Tenant::__check_create_fields(&["name"]);
            Tenant::create().name("Acme")
        }
            .exec(&mut db)
            .await?;
        {
            ::std::io::_print(format_args!("created tenant; name={0:?}\n", acme.name));
        };
        let alice = {
            let __scope = acme.users();
            struct __Check<__S: toasty::codegen_support::ValidateCreate>(
                std::marker::PhantomData<__S>,
            );
            impl<__S: toasty::codegen_support::ValidateCreate> __Check<__S> {
                const __ASSERT: () = toasty::codegen_support::assert_create_fields(
                    __S::CREATE_META,
                    &["name", "id"],
                );
            }
            fn __force_check<__S: toasty::codegen_support::ValidateCreate>(_: &__S) {
                let _ = __Check::<__S>::__ASSERT;
            }
            __force_check(&__scope);
            let __scope_fields = toasty::codegen_support::scope_fields(&__scope);
            __scope
                .create()
                .name("Alice")
                .id(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}", Uuid::new_v4()))
                    }),
                )
        }
            .exec(&mut db)
            .await?;
        let bob = {
            let __scope = acme.users();
            struct __Check<__S: toasty::codegen_support::ValidateCreate>(
                std::marker::PhantomData<__S>,
            );
            impl<__S: toasty::codegen_support::ValidateCreate> __Check<__S> {
                const __ASSERT: () = toasty::codegen_support::assert_create_fields(
                    __S::CREATE_META,
                    &["name", "id"],
                );
            }
            fn __force_check<__S: toasty::codegen_support::ValidateCreate>(_: &__S) {
                let _ = __Check::<__S>::__ASSERT;
            }
            __force_check(&__scope);
            let __scope_fields = toasty::codegen_support::scope_fields(&__scope);
            __scope
                .create()
                .name("Bob")
                .id(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}", Uuid::new_v4()))
                    }),
                )
        }
            .exec(&mut db)
            .await?;
        {
            ::std::io::_print(
                format_args!(
                    "created users; alice={0:?}, bob={1:?}\n",
                    alice.name,
                    bob.name,
                ),
            );
        };
        {
            ::std::io::_print(format_args!("====================\n"));
        };
        {
            ::std::io::_print(format_args!("--- ACME USERS ---\n"));
        };
        {
            ::std::io::_print(format_args!("====================\n"));
        };
        let users = acme.users().exec(&mut db).await?;
        for user in users {
            {
                ::std::io::_print(format_args!("USER name={0:?}\n", user.name));
            };
        }
        let todo = Todo::create()
            .user(&alice)
            .title("A Title")
            .id(
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("{0}", Uuid::new_v4()))
                }),
            )
            .exec(&mut db)
            .await?;
        let mut users = User::filter_by_tenant_id(acme.id)
            .filter_by_id(&alice.id)
            .include(User::fields().todos())
            .exec(&mut db)
            .await?;
        let from_db = users.pop().expect("Should have found a user");
        {
            ::std::io::_print(format_args!("User with include todos: {0:?}\n", from_db));
        };
        match (&1, &from_db.todos.get().len()) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        Ok(())
    };
    let body = {
        if false {
            let _: &dyn ::core::future::Future<Output = toasty::Result<()>> = &body;
        }
        body
    };
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return,
        clippy::unwrap_in_result
    )]
    {
        use tokio::runtime::Builder;
        return Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
