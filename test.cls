/*
    Copyright (c) 2014, Salesforce.org
    All rights reserved.

    Redistribution and use in source and binary forms, with or without
    modification, are permitted provided that the following conditions are met:

    * Redistributions of source code must retain the above copyright
      notice, this list of conditions and the following disclaimer.
    * Redistributions in binary form must reproduce the above copyright
      notice, this list of conditions and the following disclaimer in the
      documentation and/or other materials provided with the distributio
    * Neither the name of Salesforce.org nor the names of
      its contributors may be used to endorse or promote products derived
      from this software without specific prior written permissio

    THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
    "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
    LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
    FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
    COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT,
    INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
    BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
    LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
    CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
    LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN
    ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
    POSSIBILITY OF SUCH DAMAGE.
*/
/**
* @author Salesforce.org
* @date 2014
* @group Account Model Converter
* @description The controller for the VF page invoked by using the bulk Account model conversion utility.  This controller handles basic logic for checking the user setup and any invalid states prior to allowing the utility to run.
*/

public with sharing class CONV_Account_Conversion_CTRL {

    /**
     * @description Exception thrown when conversion process cannot go ahead
     */
    public class ConversionException extends Exception { }

    /***
    * @description Verifies the user has Admin permissions for Opportunity naming settings.
    * @return true when the user has the appropriate access
    */
    public Boolean isRunningUserIsAdmin {
        get {
            if (isRunningUserIsAdmin == null) {
                isRunningUserIsAdmin = STG_Panel.runningUserIsAdmin();
            }
            return isRunningUserIsAdmin;
        }
        private set;
    }


    /*******************************************************************************************************
    * @description Set to true before the batch job is envoked
    */
    public Boolean batchRunning { get; set; }
    /*******************************************************************************************************
    * @description Bound to the picklist for selecting field for storing the original Household Id
    */
    public String AccountHouseholdIdField {get;set;}
    /*******************************************************************************************************
    * @description Determines if the page is rendered displaying an error state
    */
    public Boolean runCheckErrorState {get;set;}
    /*******************************************************************************************************
    * @description List of select options displayed to the user when choosing AccountHouseholdIdField field
    */
    public List<SelectOption> targetAccountfields = new list<SelectOption>();
    /*******************************************************************************************************
    * @description List of async jobs to show progress to the user as job is being run
    */
    private List<AsyncApexJob> aaj = new List<AsyncApexJob>();
    /*******************************************************************************************************
    * @description Displays a job to the user as the progress meter
    */
    public AsyncApexJob apexjob {get{
            if (aaj.isEmpty())
                aaj = [select a.Id, a.TotalJobItems, a.Status, a.NumberOfErrors,
                                a.JobType, a.JobItemsProcessed, a.ExtendedStatus,
                                a.CompletedDate, a.ApexClassId From AsyncApexJob a
                                where (a.Status = 'Completed' or a.Status = 'Aborted' or a.Status = 'Failed') and a.JobType = 'BatchApex'
                                order by CompletedDate desc limit 1];
        return !aaj.isEmpty() ? aaj[0] : null;
    }set;}

    /*******************************************************************************************************
    * @description Parameter for the page passed via URL.  Currently supports 'scopsize' only
    */
    private Map<String, String> pageParams = new Map<String, String>();
    /*******************************************************************************************************
    * @description Default scope size for the batch job. Can be overridden by passing the 'scopesize' param
    */
    private Integer scopesize = 200;
    /*******************************************************************************************************
    * @description Getter for list of possible fields to store Household Account Id displayed on the page
    * @return List<SelectOption>
    */
    public list<SelectOption> getTargetAccountFields(){
        targetAccountfields.clear();
        Map<string, Schema.Sobjectfield> targetTokenMap = new Map<string, Schema.Sobjectfield>();
        targetTokenMap = Schema.SObjectType.Account.fields.getMap().clone();

        //pop out the known fields
        for (String s : AccountFields){
            s = s.toLowerCase();
            targetTokenMap.remove(s);
        }
        targetAccountfields.add(new SelectOption('', '-None-'));

        for (String s : targetTokenMap.keyset()){
            if (s.endsWith('__c')){
                List<String> nameSegs = s.split('__');
                if (nameSegs.size() < 3){
                    Schema.DescribeFieldResult f = targetTokenMap.get(s).getDescribe();
                    Schema.DisplayType payType = f.getType();
                    if(payType == Schema.DisplayType.Id || payType == Schema.DisplayType.TextArea || payType == Schema.DisplayType.String){
                        targetAccountfields.add(new SelectOption(f.getName(), s));
                    }
                }
            }
        }
        return targetAccountfields;
    }
    /*******************************************************************************************************
    * @description Set of fields to query from the original npo02__Household__c object.  Passed to the query
    */
    private static Set<String> HouseholdFields = new Set<String>{'Name', +
        'npo02__TotalOppAmount__c', 'npo02__TotalMembershipOppAmount__c', 'npo02__SmallestAmount__c', +
        'npo02__SYSTEM_CUSTOM_NAMING__c', 'npo02__OppsClosedThisYear__c', 'npo02__OppsClosedLastYear__c', +
        'npo02__OppsClosedLastNDays__c', 'npo02__OppsClosed2YearsAgo__c', 'npo02__OppAmountThisYear__c', 'npo02__OppAmountLastYear__c', +
        'npo02__OppAmountLastNDays__c', 'npo02__OppAmount2YearsAgo__c', 'npo02__NumberOfMembershipOpps__c', 'npo02__NumberOfClosedOpps__c', +
        'npo02__Membership_Status__c', 'npo02__Membership_Span__c', 'npo02__MembershipJoinDate__c', 'npo02__MembershipEndDate__c',  +
        'npo02__MailingStreet__c', 'npo02__MailingState__c', 'npo02__MailingPostalCode__c', 'npo02__MailingCountry__c', +
        'npo02__MailingCity__c', 'npo02__LastOppAmount__c', 'npo02__LastMembershipOrigin__c', 'npo02__LastMembershipLevel__c', +
        'npo02__LastMembershipDate__c', 'npo02__LastMembershipAmount__c', 'npo02__LastCloseDate__c', 'npo02__LargestAmount__c', +
        'npo02__Informal_Greeting__c', 'npo02__Household_ID__c', 'npo02__HouseholdPhone__c', 'npo02__HouseholdEmail__c', +
        'npo02__Formula_MailingAddress__c', 'npo02__Formal_Greeting__c', 'npo02__FirstCloseDate__c', 'npo02__Best_Gift_Year__c', +
        'npo02__Best_Gift_Year_Total__c', 'npo02__AverageAmount__c', 'npo02__Always_Anonymous__c', 'npo02__Addressee__c', +
        'OwnerId ', 'Owner.IsActive', 'CreatedById', 'CreatedDate', 'LastModifiedById', 'LastModifiedDate'
    };
    /*******************************************************************************************************
    * @description Set of Account fields invalid for storing the Household Account Id - helps to cull list displayed to the user
    */
    private static Set<String> AccountFields = new Set<String>{
        'npo02__TotalOppAmount__c', 'npo02__TotalMembershipOppAmount__c', 'npo02__SmallestAmount__c', 'npo02__SYSTEM_CUSTOM_NAMING__c', 'npo02__OppsClosedThisYear__c', +
        'npo02__OppsClosedLastYear__c', 'npo02__OppsClosedLastNDays__c', 'npo02__OppsClosed2YearsAgo__c', 'npo02__OppAmountThisYear__c', 'npo02__OppAmountLastYear__c', +
        'npo02__OppAmountLastNDays__c', 'npo02__OppAmount2YearsAgo__c', 'npo02__NumberOfMembershipOpps__c', 'npo02__NumberOfClosedOpps__c', 'npo02__MembershipJoinDate__c', +
        'npo02__MembershipEndDate__c', 'npo02__LastOppAmount__c', 'npo02__LastMembershipOrigin__c', 'npo02__LastMembershipLevel__c', 'npo02__LastMembershipDate__c', +
        'npo02__LastMembershipAmount__c', 'npo02__LastCloseDate__c', 'npo02__LargestAmount__c', 'npo02__Informal_Greeting__c', 'npo02__HouseholdPhone__c', 'npo02__Formal_Greeting__c', +
        'npo02__FirstCloseDate__c', 'npo02__Best_Gift_Year__c', 'npo02__Best_Gift_Year_Total__c', 'npo02__AverageAmount__c', 'npe01__SYSTEM_AccountType__c', 'npe01__SYSTEMIsIndividual__c', +
        'npe01__One2OneContact__c', 'npe01__LifetimeDonationHistory_Number__c', 'npe01__LifetimeDonationHistory_Amount__c', 'npe01__LastDonationDate__c', 'npe01__FirstDonationDate__c'
    };

    /*******************************************************************************************************
    * @description Constructs the page.  Sets defaults, gets params from URL if any
    */
    public CONV_Account_Conversion_CTRL() {

        batchRunning = false;
        runCheckErrorState = false;
        pageParams = ApexPages.currentPage().getParameters();

        if (pageParams.containsKey('scopesize')){
            try{
                scopesize = Integer.valueOf(pageParams.get('scopesize'));
            }
            catch (Exception e){
                ApexPages.addMessage(new ApexPages.Message(ApexPages.Severity.ERROR, 'Could not retrieve scope size from the URL.  Please check that the scope size is in the form of an integer'));
            }
        }
    }

    /*******************************************************************************************************
    * @description Envokes the convert action when the button is pressed. Gets the query, instantiates the batch job and run executeBatch
    * @return PageReference
    */
    public PageReference convert() {
        try {
            if (!isRunningUserIsAdmin) {
                batchRunning = false;
                throw new ConversionException(System.Label.CONV_ConversionAccessDenied);
            }

            batchRunning = true;
            String query = CONV_Account_Conversion_CTRL.getBatchQuery();
            CONV_Account_Conversion_BATCH cacv = new CONV_Account_Conversion_BATCH(query, AccountHouseholdIdField, null);
            Id batchprocessid = Database.executeBatch(cacv, scopesize);

        } catch (Exception e) {
            ERR_Handler.processError(e, ERR_Handler_API.Context.CONV);
            ApexPages.addMessage(new ApexPages.Message(ApexPages.Severity.ERROR, e.getMessage()));
        }

        return null;
    }

    /*******************************************************************************************************
    * @description Sends the user back to the Home tab when the page 'cancel' button is pressed
    * @return PageReference
    */
    public PageReference cancel() {
        return USER_UserService.getHomePageReference();
    }

    /*******************************************************************************************************
    * @description Creates the query for the batch job utilizing the set of Household fields
    * @return String
    */
    public static String getBatchQuery(){
        String query ='SELECT id';
        for (String s : CONV_Account_Conversion_CTRL.HouseholdFields)
            query += ', ' + s;
        query += ' FROM npo02__Household__c n';
        if (Test.isRunningTest())
            query += ' LIMIT 200';
        return query;
    }

    /*******************************************************************************************************
    * @description Checks the status of the org data when the page is envoked. Queries Contacts, Opportunities, Accounts, Tasks and
    * Users to display warnings about existing Household Accounts, objects owned by inactive users, and record type configuration
    */
    public void runCheck(){

        if (!isRunningUserIsAdmin) {
            ApexPages.addMessage(new ApexPages.Message(ApexPages.Severity.ERROR, System.Label.CONV_ConversionAccessDenied));
            runCheckErrorState = true;
            return;
        }

        Integer HHAccountCount = 0;
        Integer ContactConnectedCount;
        String displayHHAccountCount = '';
        String displayContactCount = '';

        HHAccountCount= [select count() from Account where npe01__SYSTEM_AccountType__c = 'Household Account' and npe01__SYSTEMIsIndividual__c = true limit 9999];
        ContactConnectedCount = [select count() from Contact where Account.npe01__System_AccountType__c = 'Household Account' limit 9999];

        if (ContactConnectedCount == 9999)
            displayContactCount = '10000+';
        else
            displayContactCount = ContactConnectedCount.format();
        if (HHAccountCount == 9999)
            displayHHAccountCount = '10000+';
        else
            displayHHAccountCount = HHAccountCount.format();

        String errorString = '';
        if (HHAccountCount > 0)
            errorString += 'Your Salesforce.com Organization currently has '+ displayHHAccountCount + ' Household Account(s) already in existence.  ';
        if (ContactConnectedCount > 0)
            errorString += 'Your Salesforce.com Organization currently has ' + displayContactCount + ' Contact(s) associated with a Household Account.  ';
        errorString += 'This converter will attempt to ignore any Contacts that already have a Household Account associated.';

        if (HHAccountCount > 0 || ContactConnectedCount > 0) {
            ApexPages.addMessage(new ApexPages.Message(ApexPages.Severity.WARNING, errorString));
        }


        List<User> inactiveUsers = new List<User>();
        inactiveUsers = [select id from User where isActive = false order by LastModifiedDate desc limit 9999];
  //      Integer accountCount = [select count() from Account where OwnerId IN :inactiveUsers limit 200];
        Integer contactCount = [select count() from Contact where OwnerId IN :inactiveUsers limit 200];
  //      Integer opptyCount = [select count() from Opportunity where OwnerId IN :inactiveUsers limit 200];
        Integer taskCount = [select count() from Task where OwnerId IN :inactiveUsers limit 200];

        String inactiveUserWarning = 'You have records owned by inactive Users.  You will need to transfer these records to an active user prior to running the conversion process.  The conversion process will not be available until these records have been reassigned.';
//        if (accountCount > 0)
  //          inactiveUserWarning += '\n ' + accountCount + ' Accounts are currently owned by an inactive user. ';
        if (contactCount > 0)
            inactiveUserWarning += '\n ' + contactCount + ' Contacts are currently owned by an inactive user. ';
      //  if (opptyCount > 0)
    //        inactiveUserWarning += '\n ' + opptyCount + ' Opportunities are currently owned by an inactive user. ';
        if (taskCount > 0)
            inactiveUserWarning += '\n ' + taskCount + ' Tasks are currently owned by an inactive user. ';

        if ((contactCount + taskCount /* accountCount + opptyCount */) > 0){
            ApexPages.addMessage(new ApexPages.Message(ApexPages.Severity.ERROR, inactiveUserWarning));
            runCheckErrorState = true;
        }

        //set the recordtype if its not already set
        Recordtypeinfo householdAccount =  Schema.SObjectType.Account.getRecordTypeInfosByDeveloperName().get(CAO_Constants.HH_ACCOUNT_RT_DEVELOPER_NAME);
        if (UTIL_CustomSettingsFacade.getContactsSettings().npe01__HH_Account_RecordTypeID__c == null && householdAccount != null){
            npe01__Contacts_and_Orgs_Settings__c ContactsSettings = UTIL_CustomSettingsFacade.getContactsSettings();
            ContactsSettings.npe01__HH_Account_RecordTypeID__c = householdAccount.getRecordTypeId();
            if(!Test.isRunningTest())
                upsert ContactsSettings;
        }

        //check the running user's profile access to the account rec type to be used for HH accounts
        if (!Schema.Sobjecttype.Account.getRecordTypeInfosById().get(UTIL_CustomSettingsFacade.getContactsSettings().npe01__HH_Account_RecordTypeID__c).isAvailable()){
            ApexPages.addMessage(new ApexPages.Message(ApexPages.Severity.ERROR, 'The currently selected Household Account recordtype is not accessible to your current profile.  You will need to modify your profile to include this record type prior to running the conversion process. The conversion process will not be available until the record type is made available to your profile, or a different record type is set in the NPSP Settings tab.'));
            runCheckErrorState = true;
        }
    }

    /*******************************************************************************************************
    * @description Tests the field selected by the user to ensure its a valid target for the Household Id
    * @param HHField User selected Id field to check
    * @return Boolean
    */
    @RemoteAction
    public static Boolean verifyFieldTypeForIds(String HHField){
        Map<string, Schema.Sobjectfield> targetTokenMap = new Map<string, Schema.Sobjectfield>();
        targetTokenMap = Schema.SObjectType.Account.fields.getMap().clone();

        if (targetTokenMap.containsKey(HHField)){
            Schema.DescribeFieldResult f = targetTokenMap.get(HHField).getDescribe();
            Schema.DisplayType payType = f.getType();
            if(payType == Schema.DisplayType.Id || payType == Schema.DisplayType.TextArea || payType == Schema.DisplayType.String)
                return true;
            else
                return false;
        }
        else{
            return false;
        }
    }
}
