use super::*;

#[test]
fn remove_at_entity_property_vector_success() {
    with_test_externalities(|| {
        let actor = Actor::Lead;

        // Add entity schemas support
        let (mut first_entity, mut second_entity) = add_entity_schemas_support();

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_calls = System::events().len();

        // Remove value at given `index_in_property_vector`
        // from `PropertyValueVec` under in_class_schema_property_id
        let nonce = 0;
        let index_in_property_vector = 0;

        assert_ok!(remove_at_entity_property_vector(
            LEAD_ORIGIN,
            actor.clone(),
            FIRST_ENTITY_ID,
            SECOND_PROPERTY_ID,
            index_in_property_vector,
            nonce
        ));

        // Runtime tested state after call

        // Ensure first entity properties updated succesfully
        if let Some(second_schema_old_property_value) = first_entity
            .get_values_mut()
            .get_mut(&SECOND_PROPERTY_ID)
            .and_then(|property_value| property_value.as_vec_property_value_mut())
        {
            second_schema_old_property_value.remove_at(index_in_property_vector);
        }

        assert_eq!(first_entity, entity_by_id(FIRST_ENTITY_ID));

        // Ensure reference counter of second entity updated succesfully
        let inbound_rc = InboundReferenceCounter::new(2, true);
        *second_entity.get_reference_counter_mut() = inbound_rc;

        assert_eq!(second_entity, entity_by_id(SECOND_ENTITY_ID));

        // Create side-effect
        let side_effect = EntityReferenceCounterSideEffect::new(-1, -1);

        let removed_at_vector_index_event = get_test_event(RawEvent::RemovedAtVectorIndex(
            actor,
            FIRST_ENTITY_ID,
            SECOND_PROPERTY_ID,
            index_in_property_vector,
            nonce + 1,
            Some((SECOND_ENTITY_ID, side_effect)),
        ));

        // Last event checked
        assert_event(
            removed_at_vector_index_event,
            number_of_events_before_calls + 1,
        );
    })
}

#[test]
fn remove_at_entity_property_vector_entity_not_found() {
    with_test_externalities(|| {
        let actor = emulate_entity_access_state_for_failure_case(
            EntityAccessStateFailureType::EntityNotFound,
        );

        // Create class reference schema
        add_unique_class_reference_schema();

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        let nonce = 0;
        let index_in_property_vector = 0;

        // Make an attempt to remove value at given `index_in_property_vector`
        // from `PropertyValueVec` under `in_class_schema_property_id` in case when corresponding Entity does not exist
        let remove_at_entity_property_vector_result = remove_at_entity_property_vector(
            LEAD_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_PROPERTY_ID,
            index_in_property_vector,
            nonce,
        );

        // Failure checked
        assert_failure(
            remove_at_entity_property_vector_result,
            Error::<Runtime>::EntityNotFound,
            number_of_events_before_call,
        );
    })
}

#[test]
fn remove_at_entity_property_vector_lead_auth_failed() {
    with_test_externalities(|| {
        let actor = emulate_entity_access_state_for_failure_case(
            EntityAccessStateFailureType::LeadAuthFailed,
        );

        // Create class reference schema and add corresponding schema support to the Entity
        add_unique_class_reference_schema_and_entity_schema_support(&actor, LEAD_ORIGIN);

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        let nonce = 0;
        let index_in_property_vector = 0;

        // Make an attempt to remove value at given `index_in_property_vector`
        // from `PropertyValueVec` under `in_class_schema_property_id` using unknown origin and lead actor
        let remove_at_entity_property_vector_result = remove_at_entity_property_vector(
            UNKNOWN_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_PROPERTY_ID,
            index_in_property_vector,
            nonce,
        );

        // Failure checked
        assert_failure(
            remove_at_entity_property_vector_result,
            Error::<Runtime>::LeadAuthFailed,
            number_of_events_before_call,
        );
    })
}

#[test]
fn remove_at_entity_property_vector_member_auth_failed() {
    with_test_externalities(|| {
        let actor = emulate_entity_access_state_for_failure_case(
            EntityAccessStateFailureType::MemberAuthFailed,
        );

        // Create class reference schema and add corresponding schema support to the Entity
        add_unique_class_reference_schema_and_entity_schema_support(&actor, LEAD_ORIGIN);

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        let nonce = 0;
        let index_in_property_vector = 0;

        // Make an attempt to remove value at given `index_in_property_vector`
        // from `PropertyValueVec` under `in_class_schema_property_id` using unknown origin and member actor
        let remove_at_entity_property_vector_result = remove_at_entity_property_vector(
            UNKNOWN_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_PROPERTY_ID,
            index_in_property_vector,
            nonce,
        );

        // Failure checked
        assert_failure(
            remove_at_entity_property_vector_result,
            Error::<Runtime>::MemberAuthFailed,
            number_of_events_before_call,
        );
    })
}

#[test]
fn remove_at_entity_property_vector_curator_group_is_not_active() {
    with_test_externalities(|| {
        let actor = emulate_entity_access_state_for_failure_case(
            EntityAccessStateFailureType::CuratorAuthFailed,
        );

        // Create class reference schema and add corresponding schema support to the Entity
        add_unique_class_reference_schema_and_entity_schema_support(&actor, FIRST_CURATOR_ORIGIN);

        // Make curator group inactive to block it from any entity operations
        assert_ok!(set_curator_group_status(
            LEAD_ORIGIN,
            FIRST_CURATOR_GROUP_ID,
            false
        ));

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        let nonce = 0;
        let index_in_property_vector = 0;

        // Make an attempt to remove value at given `index_in_property_vector`
        // from `PropertyValueVec` under `in_class_schema_property_id` using curator group, which is not active as actor
        let remove_at_entity_property_vector_result = remove_at_entity_property_vector(
            FIRST_CURATOR_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_PROPERTY_ID,
            index_in_property_vector,
            nonce,
        );

        // Failure checked
        assert_failure(
            remove_at_entity_property_vector_result,
            Error::<Runtime>::CuratorGroupIsNotActive,
            number_of_events_before_call,
        );
    })
}

#[test]
fn remove_at_entity_property_vector_curator_auth_failed() {
    with_test_externalities(|| {
        let actor = emulate_entity_access_state_for_failure_case(
            EntityAccessStateFailureType::CuratorAuthFailed,
        );

        // Create class reference schema and add corresponding schema support to the Entity
        add_unique_class_reference_schema_and_entity_schema_support(&actor, FIRST_CURATOR_ORIGIN);

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        let nonce = 0;
        let index_in_property_vector = 0;

        // Make an attempt to remove value at given `index_in_property_vector`
        // from `PropertyValueVec` under `in_class_schema_property_id` using unknown origin and curator actor
        let remove_at_entity_property_vector_result = remove_at_entity_property_vector(
            UNKNOWN_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_PROPERTY_ID,
            index_in_property_vector,
            nonce,
        );

        // Failure checked
        assert_failure(
            remove_at_entity_property_vector_result,
            Error::<Runtime>::CuratorAuthFailed,
            number_of_events_before_call,
        );
    })
}

#[test]
fn remove_at_entity_property_vector_curator_not_found_in_curator_group() {
    with_test_externalities(|| {
        let actor = emulate_entity_access_state_for_failure_case(
            EntityAccessStateFailureType::CuratorNotFoundInCuratorGroup,
        );

        // Create class reference schema and add corresponding schema support to the Entity
        add_unique_class_reference_schema_and_entity_schema_support(
            &Actor::Curator(FIRST_CURATOR_GROUP_ID, FIRST_CURATOR_ID),
            FIRST_CURATOR_ORIGIN,
        );

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        let nonce = 0;
        let index_in_property_vector = 0;

        // Make an attempt to remove value at given `index_in_property_vector`
        // from `PropertyValueVec` under `in_class_schema_property_id`,
        // using actor in group, which curator id was not added to corresponding group set
        let remove_at_entity_property_vector_result = remove_at_entity_property_vector(
            SECOND_CURATOR_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_PROPERTY_ID,
            index_in_property_vector,
            nonce,
        );

        // Failure checked
        assert_failure(
            remove_at_entity_property_vector_result,
            Error::<Runtime>::CuratorIsNotAMemberOfGivenCuratorGroup,
            number_of_events_before_call,
        );
    })
}

#[test]
fn remove_at_entity_property_vector_access_denied() {
    with_test_externalities(|| {
        let actor = emulate_entity_access_state_for_failure_case(
            EntityAccessStateFailureType::EntityAccessDenied,
        );

        // Create class reference schema and add corresponding schema support to the Entity
        add_unique_class_reference_schema_and_entity_schema_support(&Actor::Lead, LEAD_ORIGIN);

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        let nonce = 0;
        let index_in_property_vector = 0;

        // Make an attempt to remove value at given `index_in_property_vector`
        // from `PropertyValueVec` under `in_class_schema_property_id`,
        // using origin, which corresponding actor is neither entity maintainer, nor controller.
        let remove_at_entity_property_vector_result = remove_at_entity_property_vector(
            SECOND_CURATOR_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_PROPERTY_ID,
            index_in_property_vector,
            nonce,
        );

        // Failure checked
        assert_failure(
            remove_at_entity_property_vector_result,
            Error::<Runtime>::EntityAccessDenied,
            number_of_events_before_call,
        );
    })
}

#[test]
fn remove_at_entity_property_vector_values_locked_on_class_level() {
    with_test_externalities(|| {
        let actor = emulate_entity_access_state_for_failure_case(
            EntityAccessStateFailureType::PropertyValuesLocked,
        );

        // Create class reference schema and add corresponding schema support to the Entity
        add_unique_class_reference_schema_and_entity_schema_support(&Actor::Lead, LEAD_ORIGIN);

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        let nonce = 0;
        let index_in_property_vector = 0;

        // Make an attempt to remove value at given `index_in_property_vector`
        // from `PropertyValueVec` under `in_class_schema_property_id`,
        // in the case, when all property values were locked on Class level.
        let remove_at_entity_property_vector_result = remove_at_entity_property_vector(
            LEAD_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_PROPERTY_ID,
            index_in_property_vector,
            nonce,
        );

        // Failure checked
        assert_failure(
            remove_at_entity_property_vector_result,
            Error::<Runtime>::AllPropertiesWereLockedOnClassLevel,
            number_of_events_before_call,
        );
    })
}

#[test]
fn remove_at_entity_property_vector_class_property_not_found() {
    with_test_externalities(|| {
        // Create class with default permissions
        assert_ok!(create_simple_class(LEAD_ORIGIN, ClassType::Valid));

        let actor = Actor::Lead;

        // Create entity
        assert_ok!(create_entity(LEAD_ORIGIN, FIRST_CLASS_ID, actor.clone()));

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        let nonce = 0;
        let index_in_property_vector = 0;

        // Make an attempt to remove value at given `index_in_property_vector`
        // from `PropertyValueVec` under `in_class_schema_property_id`,
        // in the case, when Property under corresponding PropertyId was not found on Class level.
        let remove_at_entity_property_vector_result = remove_at_entity_property_vector(
            LEAD_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_PROPERTY_ID,
            index_in_property_vector,
            nonce,
        );

        // Failure checked
        assert_failure(
            remove_at_entity_property_vector_result,
            Error::<Runtime>::ClassPropertyNotFound,
            number_of_events_before_call,
        );
    })
}

#[test]
fn remove_at_entity_property_vector_is_locked_for_given_actor() {
    with_test_externalities(|| {
        // Create class with default permissions
        assert_ok!(create_simple_class(LEAD_ORIGIN, ClassType::Valid));

        // Create property
        let property_type = PropertyType::<ClassId>::vec_reference(FIRST_CLASS_ID, true, 5);

        let mut property = Property::<ClassId>::with_name_and_type(
            (PropertyNameLengthConstraint::get().max() - 1) as usize,
            property_type,
            true,
            false,
        );

        property.locking_policy = PropertyLockingPolicy::new(false, true);

        // Add Schema to the Class
        assert_ok!(add_class_schema(
            LEAD_ORIGIN,
            FIRST_CLASS_ID,
            BTreeSet::new(),
            vec![property]
        ));

        let actor = Actor::Lead;

        // Create entity
        assert_ok!(create_entity(LEAD_ORIGIN, FIRST_CLASS_ID, actor.clone()));

        let schema_property_value =
            InputPropertyValue::<Runtime>::vec_reference(vec![FIRST_ENTITY_ID, FIRST_ENTITY_ID]);

        let mut schema_property_values = BTreeMap::new();
        schema_property_values.insert(FIRST_PROPERTY_ID, schema_property_value);

        // Add schema support to the entity
        assert_ok!(add_schema_support_to_entity(
            LEAD_ORIGIN,
            actor.clone(),
            FIRST_ENTITY_ID,
            FIRST_SCHEMA_ID,
            schema_property_values
        ));

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        let nonce = 0;
        let index_in_property_vector = 0;

        // Make an attempt to remove value at given `index_in_property_vector`
        // from `PropertyValueVec` under `in_class_schema_property_id`,
        // under lead origin, which is current Entity controller, in the case,
        // when corresponding class Property was locked from controller on Class level
        let remove_at_entity_property_vector_result = remove_at_entity_property_vector(
            LEAD_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_PROPERTY_ID,
            index_in_property_vector,
            nonce,
        );

        // Failure checked
        assert_failure(
            remove_at_entity_property_vector_result,
            Error::<Runtime>::ClassPropertyTypeLockedForGivenActor,
            number_of_events_before_call,
        );
    })
}

#[test]
fn remove_at_entity_property_vector_unknown_entity_property_id() {
    with_test_externalities(|| {
        // Create class with default permissions
        assert_ok!(create_simple_class(LEAD_ORIGIN, ClassType::Valid));

        // Create class reference schema
        add_unique_class_reference_schema();

        let actor = Actor::Lead;

        // Create entity
        assert_ok!(create_entity(LEAD_ORIGIN, FIRST_CLASS_ID, actor.clone()));

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        let nonce = 0;
        let index_in_property_vector = 0;

        // Make an attempt to remove value at given `index_in_property_vector`
        // from `PropertyValueVec` under `in_class_schema_property_id`,
        // in the case, when property value was not added to current Entity values yet.
        let remove_at_entity_property_vector_result = remove_at_entity_property_vector(
            LEAD_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_PROPERTY_ID,
            index_in_property_vector,
            nonce,
        );

        // Failure checked
        assert_failure(
            remove_at_entity_property_vector_result,
            Error::<Runtime>::UnknownEntityPropertyId,
            number_of_events_before_call,
        );
    })
}

#[test]
fn remove_at_entity_property_vector_value_under_given_index_is_not_a_vector() {
    with_test_externalities(|| {
        // Create class with default permissions
        assert_ok!(create_simple_class(LEAD_ORIGIN, ClassType::Valid));

        // Create property
        let property = Property::<ClassId>::default_with_name(
            PropertyNameLengthConstraint::get().max() as usize,
        );

        // Add Schema to the Class
        assert_ok!(add_class_schema(
            LEAD_ORIGIN,
            FIRST_CLASS_ID,
            BTreeSet::new(),
            vec![property]
        ));

        let actor = Actor::Lead;

        // Create entity
        assert_ok!(create_entity(LEAD_ORIGIN, FIRST_CLASS_ID, actor.clone()));

        let mut schema_property_values = BTreeMap::new();
        schema_property_values.insert(FIRST_PROPERTY_ID, InputPropertyValue::default());

        // Add schema support to the entity
        assert_ok!(add_schema_support_to_entity(
            LEAD_ORIGIN,
            actor.clone(),
            FIRST_ENTITY_ID,
            FIRST_SCHEMA_ID,
            schema_property_values,
        ));

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        let nonce = 0;
        let index_in_property_vector = 0;

        // Make an attempt to remove value at given `index_in_property_vector`
        // from `PropertyValueVec` under `in_class_schema_property_id`,
        // in the case, when entity property value corresponding to a given in_class_schema_property_id is not a vector.
        let remove_at_entity_property_vector_result = remove_at_entity_property_vector(
            LEAD_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_PROPERTY_ID,
            index_in_property_vector,
            nonce,
        );

        // Failure checked
        assert_failure(
            remove_at_entity_property_vector_result,
            Error::<Runtime>::PropertyValueUnderGivenIndexIsNotAVector,
            number_of_events_before_call,
        );
    })
}

#[test]
fn remove_at_entity_property_vector_nonces_does_not_match() {
    with_test_externalities(|| {
        // Create class with default permissions
        assert_ok!(create_simple_class(LEAD_ORIGIN, ClassType::Valid));

        let actor = Actor::Lead;

        // Create entity
        assert_ok!(create_entity(LEAD_ORIGIN, FIRST_CLASS_ID, actor.clone()));

        // Create class reference schema and add corresponding schema support to the Entity
        add_unique_class_reference_schema_and_entity_schema_support(&Actor::Lead, LEAD_ORIGIN);

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        let nonce = 1;
        let index_in_property_vector = 0;

        // Make an attempt to remove value at given `index_in_property_vector`
        // from `PropertyValueVec` under `in_class_schema_property_id`,
        // providing nonce that does not corresponding property value vector one.
        let remove_at_entity_property_vector_result = remove_at_entity_property_vector(
            LEAD_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_PROPERTY_ID,
            index_in_property_vector,
            nonce,
        );

        // Failure checked
        assert_failure(
            remove_at_entity_property_vector_result,
            Error::<Runtime>::PropertyValueVecNoncesDoesNotMatch,
            number_of_events_before_call,
        );
    })
}

#[test]
fn remove_at_entity_property_vector_index_is_out_of_range() {
    with_test_externalities(|| {
        // Create class with default permissions
        assert_ok!(create_simple_class(LEAD_ORIGIN, ClassType::Valid));

        let actor = Actor::Lead;

        // Create entity
        assert_ok!(create_entity(LEAD_ORIGIN, FIRST_CLASS_ID, actor.clone()));

        // Create class reference schema
        add_unique_class_reference_schema();

        let entity_ids = vec![FIRST_ENTITY_ID, FIRST_ENTITY_ID];
        let schema_property_value =
            InputPropertyValue::<Runtime>::vec_reference(entity_ids.clone());

        let mut schema_property_values = BTreeMap::new();
        schema_property_values.insert(FIRST_PROPERTY_ID, schema_property_value);

        // Add schema support to the entity
        assert_ok!(add_schema_support_to_entity(
            LEAD_ORIGIN,
            actor.to_owned(),
            FIRST_ENTITY_ID,
            FIRST_SCHEMA_ID,
            schema_property_values
        ));

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        // Make an attempt to remove value at given `index_in_property_vector`
        // from `PropertyValueVec` under `in_class_schema_property_id` in case,
        // when provided index_in_property_vector is out of range of the related vector
        let nonce = 0;
        let index_in_property_vector = entity_ids.len() as u16 + 1;

        let remove_at_entity_property_vector_result = remove_at_entity_property_vector(
            LEAD_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_PROPERTY_ID,
            index_in_property_vector,
            nonce,
        );

        // Failure checked
        assert_failure(
            remove_at_entity_property_vector_result,
            Error::<Runtime>::EntityPropertyValueVectorIndexIsOutOfRange,
            number_of_events_before_call,
        );
    })
}

#[test]
fn remove_at_entity_property_vector_property_should_be_unique() {
    with_test_externalities(|| {
        // Create class with default permissions
        assert_ok!(create_simple_class(LEAD_ORIGIN, ClassType::Valid));

        let actor = Actor::Lead;

        // Create first Entity
        assert_ok!(create_entity(LEAD_ORIGIN, FIRST_CLASS_ID, actor.clone()));

        // Create unique class reference schema and add corresponding schema support to the first Entity
        add_unique_class_reference_schema_and_entity_schema_support(&actor, LEAD_ORIGIN);

        // Create second Entity
        assert_ok!(create_entity(LEAD_ORIGIN, FIRST_CLASS_ID, actor.clone()));

        // Create vec reference
        let schema_property_value = InputPropertyValue::<Runtime>::vec_reference(vec![
            FIRST_ENTITY_ID,
            FIRST_ENTITY_ID,
            FIRST_ENTITY_ID,
        ]);

        let mut schema_property_values = BTreeMap::new();
        schema_property_values.insert(FIRST_PROPERTY_ID, schema_property_value);

        // Add schema support to the second Entity
        assert_ok!(add_schema_support_to_entity(
            LEAD_ORIGIN,
            actor.to_owned(),
            SECOND_ENTITY_ID,
            FIRST_SCHEMA_ID,
            schema_property_values
        ));

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        let nonce = 0;
        let index_in_property_vector = 0;

        // Make an attempt to remove value at given `index_in_property_vector`
        // from `PropertyValueVec` under `in_class_schema_property_id` in case,
        // when in result we`ll get required & unique property value vector,
        // which is already added to another Entity of this Class.
        let remove_at_entity_property_vector_result = remove_at_entity_property_vector(
            LEAD_ORIGIN,
            actor,
            SECOND_ENTITY_ID,
            FIRST_PROPERTY_ID,
            index_in_property_vector,
            nonce,
        );

        // Failure checked
        assert_failure(
            remove_at_entity_property_vector_result,
            Error::<Runtime>::PropertyValueShouldBeUnique,
            number_of_events_before_call,
        );
    })
}
