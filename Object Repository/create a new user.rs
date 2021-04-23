<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>create a new user</name>
   <tag></tag>
   <elementGuidId>d718bcfd-49f5-44a1-9709-9fd03c86ebcd</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;application/x-www-form-urlencoded&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;email&quot;,
      &quot;value&quot;: &quot;george.harisson@reqres.in&quot;
    },
    {
      &quot;name&quot;: &quot;first_name&quot;,
      &quot;value&quot;: &quot;George&quot;
    },
    {
      &quot;name&quot;: &quot;last_name&quot;,
      &quot;value&quot;: &quot;Harisson&quot;
    },
    {
      &quot;name&quot;: &quot;avatar&quot;,
      &quot;value&quot;: &quot;https://reqres.in/img/faces/1-image.jpg&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>x-www-form-urlencoded</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/x-www-form-urlencoded</value>
   </httpHeaderProperties>
   <katalonVersion>7.9.1</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://reqres.in/api/users</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 201)

assertThat(response.getStatusCode()).isEqualTo(201)

WS.verifyElementPropertyValue(response, 'email', 'george.harisson@reqres.in')
WS.verifyElementPropertyValue(response, 'first_name', 'George')
WS.verifyElementPropertyValue(response, 'last_name', 'Harisson')
WS.verifyElementPropertyValue(response, 'avatar', 'https://reqres.in/img/faces/1-image.jpg')
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
