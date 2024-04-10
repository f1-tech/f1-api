use diesel::QueryResult;
use sea_query::{Expr, Query, SelectStatement};
use sea_query_diesel::{DieselBinder, SeaQuery};

use crate::iden::*;
use shared::filters::GetCircuitsFilter;

pub struct CircuitQueryBuilder {
    stmt: SelectStatement,
    filter: GetCircuitsFilter,
}

impl CircuitQueryBuilder {
    pub fn filter(filter: GetCircuitsFilter) -> Self {
        let mut stmt = Query::select();
        stmt.distinct().from(Circuits::Table).columns(
            [
                Circuits::CircuitId,
                Circuits::CircuitRef,
                Circuits::Name,
                Circuits::Location,
                Circuits::Country,
                Circuits::Lat,
                Circuits::Lng,
                Circuits::Alt,
                Circuits::Url,
            ]
            .into_iter()
            .map(|c| (Circuits::Table, c)),
        );

        Self { stmt, filter }
    }

    pub fn build(self) -> String {
        self.stmt.to_string(sea_query::MysqlQueryBuilder)
    }

    fn one_of(&self) -> bool {
        self.filter.driver_ref.is_some()
            || self.filter.constructor_ref.is_some()
            || self.filter.circuit_ref.is_some()
            || self.filter.grid.is_some()
            || self.filter.result.is_some()
            || self.filter.year.is_some()
            || self.filter.round.is_some()
    }

    fn races_table(mut self) -> Self {
        if self.one_of() {
            self.stmt.join(
                sea_query::JoinType::Join,
                Races::Table,
                Expr::col((Circuits::Table, Circuits::CircuitId))
                    .equals((Races::Table, Races::CircuitId)),
            );
        }
        self
    }

    fn results_table(mut self) -> Self {
        if self.one_of() {
            self.stmt.join(
                sea_query::JoinType::Join,
                Results::Table,
                Expr::col((Races::Table, Races::CircuitId))
                    .equals((Results::Table, Results::RaceId)),
            );
        }
        self
    }

    fn drivers_table(mut self) -> Self {
        if self.filter.driver_ref.is_some() {
            self.stmt.join(
                sea_query::JoinType::Join,
                Drivers::Table,
                Expr::col((Results::Table, Results::DriverId))
                    .equals((Drivers::Table, Drivers::DriverId)),
            );
        }
        self
    }

    fn constructors_table(mut self) -> Self {
        if self.filter.constructor_ref.is_some() {
            self.stmt.join(
                sea_query::JoinType::Join,
                Constructors::Table,
                Expr::col((Results::Table, Results::ConstructorId))
                    .equals((Constructors::Table, Constructors::ConstructorId)),
            );
        }
        self
    }
}
